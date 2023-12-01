use weaksignal::ft8::{DecodeParams, FT8Decoder};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let mut reader = hound::WavReader::open(filename).unwrap();
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / 32768.0)
        .collect();
    let sample_rate = reader.spec().sample_rate;
    let mut params: DecodeParams = Default::default();
    params.sr = sample_rate as f32;
    params.decode_attempts = 30;
    let mut decoder = FT8Decoder::new(&params);
    let messages = decoder.decode(samples.as_slice()).unwrap();
    messages.iter().for_each(|f| println!("{}", f));
}
