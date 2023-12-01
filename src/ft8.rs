use std::{collections::VecDeque, f32::consts::PI, iter::zip, sync::Arc};

use realfft::{FftError, RealFftPlanner, RealToComplex};
use rustfft::num_complex::Complex;

use crate::{
    callsignmap::CallsignMap, constants::FT8_SYMBOLS, ldpc::ldpc_decode, message::Message,
    utils::bool_slice_to_u16,
};

pub struct DecodeParams {
    pub sr: f32,
    pub decode_attempts: u32,
    pub from_freq: f32,
    pub to_freq: f32,
}

impl Default for DecodeParams {
    fn default() -> Self {
        DecodeParams {
            sr: 12000.0,
            decode_attempts: 10,
            from_freq: 0.0,
            to_freq: 3000.0,
        }
    }
}

pub struct FT8Decoder {
    fft_size: usize,
    plan: Arc<dyn RealToComplex<f32>>,
    input: Vec<f32>,
    window: Vec<f32>,
    spectrum: Vec<Complex<f32>>,
    from_bin: usize,
    to_bin: usize,
    pwr: Vec<VecDeque<f32>>,
    symbols: Vec<VecDeque<f32>>,
    callsign_map: CallsignMap,
    decode_attempts: u32,
    pending_samples: VecDeque<f32>,
    total_samples: usize,
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

impl FT8Decoder {
    pub fn new(params: &DecodeParams) -> FT8Decoder {
        let sr = params.sr;
        let mut fft_planner = RealFftPlanner::<f32>::new();
        let fft_size = (sr * 2.0 * 1920.0 / 12000.0).round() as usize;
        let plan = fft_planner.plan_fft_forward(fft_size);
        let input = plan.make_input_vec();
        let spectrum = plan.make_output_vec();
        let window = (0..fft_size)
            .map(|i| 0.5 * (1.0 - (2.0 * PI * (i as f32) / ((fft_size - 1) as f32).cos())))
            .collect();
        FT8Decoder {
            fft_size: fft_size,
            plan: plan,
            input: input,
            window: window,
            spectrum: spectrum,
            from_bin: (params.from_freq * fft_size as f32 / sr).ceil() as usize,
            to_bin: ((params.to_freq * fft_size as f32 / sr).floor() as usize)
                .min(fft_size / 2 - 2 * 8),
            pwr: vec![VecDeque::new(); fft_size / 2 + 1],
            symbols: vec![VecDeque::new(); fft_size / 2 + 1],
            callsign_map: CallsignMap::new(),
            decode_attempts: params.decode_attempts,
            pending_samples: VecDeque::new(),
            total_samples: 0,
        }
    }

    pub fn decode(&mut self, data: &[f32]) -> Result<Vec<Message>, FftError> {
        self.pending_samples.extend(data);
        let mut messages = Vec::new();
        while self.pending_samples.len() >= self.fft_size {
            let frame_len = self.fft_size / 4;
            let mut frame = self.pending_samples.drain(0..frame_len).collect::<Vec<_>>();
            self.total_samples += frame.len();
            frame.extend(self.pending_samples.range(..self.fft_size - frame_len));
            messages.extend(self.process_frame(&frame)?);
        }
        Ok(Message::deduplicate_signals(messages))
    }

    pub fn process_frame(&mut self, frame: &[f32]) -> Result<Vec<Message>, FftError> {
        debug_assert!(frame.len() == self.fft_size);

        zip(frame, &self.window)
            .enumerate()
            .for_each(|(i, (x, w))| {
                self.input[i] = x * w;
            });

        self.plan.process(&mut self.input, &mut self.spectrum)?;

        let fft_pwr = self
            .spectrum
            .iter()
            .map(|x| (x.norm_sqr() + 1e-12).log10() * 10.0)
            .collect::<Vec<f32>>();

        for j in 0..fft_pwr.len() {
            self.pwr[j].push_back(fft_pwr[j]);
            while self.pwr[j].len() > 65 * 2 + 2 {
                self.pwr[j].pop_front();
            }
        }

        let mut messages: Vec<Message> = Vec::new();
        let mut codeword = [0.0; 174];
        for j in self.from_bin..self.to_bin {
            let mut s2 = [0.0; 8];
            for k in 0..8 {
                s2[k] = fft_pwr[j + (FT8_SYMBOLS[k] as usize) * 2];
            }
            let logl_0_1 = s2[4].max(s2[5]).max(s2[6]).max(s2[7]);
            let logl_0_0 = s2[0].max(s2[1]).max(s2[2]).max(s2[3]);
            let logl_1_1 = s2[2].max(s2[3]).max(s2[6]).max(s2[7]);
            let logl_1_0 = s2[0].max(s2[1]).max(s2[4]).max(s2[5]);
            let logl_2_1 = s2[1].max(s2[3]).max(s2[5]).max(s2[7]);
            let logl_2_0 = s2[0].max(s2[2]).max(s2[4]).max(s2[6]);
            let p0 = sigmoid(logl_0_1 - logl_0_0);
            let p1 = sigmoid(logl_1_1 - logl_1_0);
            let p2 = sigmoid(logl_2_1 - logl_2_0);

            self.symbols[j].push_back(p0);
            self.symbols[j].push_back(p1);
            self.symbols[j].push_back(p2);

            if self.symbols[j].len() > 65 * 3 * 2 {
                (0..29)
                    .chain(36..65)
                    .map(|x| x * 2)
                    .flat_map(|x| [x * 3, x * 3 + 1, x * 3 + 2])
                    .enumerate()
                    .for_each(|(i, x)| {
                        codeword[i] = self.symbols[j][x];
                    });

                ldpc_decode(&codeword, self.decode_attempts)
                    .filter(|decoded| decoded.iter().any(|&x| x))
                    .filter(|decoded| {
                        let crc: &[bool; 14] = &decoded[77..91].try_into().unwrap();
                        let data: &[bool; 77] = &decoded[..77].try_into().unwrap();
                        let decoded_crc = bool_slice_to_u16(crc) as u16;
                        Self::bitwise_crc14(data) == decoded_crc
                    })
                    .map(|decoded| {
                        let symbols = decoded
                            .chunks_exact(3)
                            .map(|x| x[0] as u8 * 4 + x[1] as u8 * 2 + x[2] as u8);

                        let symbol_pwr_idx = (0..29).chain(36..65).map(|x| x * 2);

                        let signal_noise = zip(symbols, symbol_pwr_idx)
                            .map(|(symbol, idx)| {
                                let mut signal = 0.0;
                                let mut noise = 0.0;
                                let mut s = [0.0; 8];
                                for k in 0..8 {
                                    let bin_pwr = self.pwr[j + (FT8_SYMBOLS[k] as usize) * 2][idx];
                                    s[k] = bin_pwr;
                                    if k as u8 == symbol {
                                        signal += bin_pwr;
                                    } else {
                                        noise += bin_pwr;
                                    }
                                }
                                (signal, noise)
                            })
                            .fold((0.0, 0.0), |(s, n), (s2, n2)| (s + s2, n + n2));

                        let signal = signal_noise.0 + 1e-12;
                        let noise = (signal_noise.1 + 1e-12) / 7.0;
                        let snr = ((signal - noise) / noise).log10() * 10.0;

                        let data: &[bool; 77] = &decoded[..77].try_into().unwrap();
                        Message::from_bits(
                            snr,
                            (j * 625 / 200) as u32,
                            (self.total_samples as i64 - 71 * (self.fft_size as i64) / 2) * 1000
                                / 12000,
                            data,
                            &self.callsign_map,
                        )
                    })
                    .into_iter()
                    .for_each(|message| messages.push(message));

                while self.symbols[j].len() > 65 * 3 * 2 {
                    self.symbols[j].pop_front();
                }
            }
        }
        Ok(messages)
    }

    pub fn decode_messages(&mut self, data: &[f32]) -> Result<Vec<Message>, FftError> {
        let fft_size: usize = self.fft_size;
        let blocks = data.len() / fft_size;
        let mut messages = Vec::new();
        for i in 0..blocks {
            let sample_input: &[f32] = &data[i * fft_size..(i + 1) * fft_size];
            messages.extend(self.process_frame(sample_input)?);
        }
        Ok(messages)
    }

    pub fn insert_callsign(&mut self, callsign: &String) -> u32 {
        self.callsign_map.insert(callsign)
    }
    fn bitwise_crc14(bits: &[bool; 77]) -> u16 {
        let mut crc = 0;
        let poly = 0x6757;
        for k in 0..77 {
            crc = (crc << 1) | if bits[k] { 1 } else { 0 };
            if crc & (1 << 14) != 0 {
                crc ^= poly;
            }
        }
        for _ in 0..19 {
            crc = crc << 1;
            if crc & (1 << 14) != 0 {
                crc ^= poly;
            }
        }
        crc
    }
}
