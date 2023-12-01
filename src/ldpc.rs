use crate::{
    constants::{FT8_MN, FT8_MNV, FT8_NM, FT8_NMC},
    utils::{optimal_leave_one_out_multiplication_6, optimal_leave_one_out_multiplication_7},
};

pub(crate) fn   ldpc_decode(bits: &[f32; 174], decode_times: u32) -> Option<[bool; 174]> {
    let mut v = [[0.0; 3]; 174];
    let mut c = [[0.0; 7]; 83];
    let mut output = [false; 174];
    let mut mults = [0.0; 7];
    for i in 0..174 {
        for j in 0..3 {
            v[i][j] = bits[i];
        }
    }
    for _ in 0..decode_times {
        for check in 0..83 {
            for i in 0..6 {
                let var = FT8_NM[check][i];
                let check_id = FT8_NMC[check][i];
                mults[i] = 1.0 - 2.0 * v[var as usize][check_id as usize];
            }
            let var6 = FT8_NM[check][6];
            let result = if var6 != 255 {
                let check_id = FT8_NMC[check][6];
                mults[6] = 1.0 - 2.0 * v[var6 as usize][check_id as usize];
                optimal_leave_one_out_multiplication_7(&mults)
            } else {
                optimal_leave_one_out_multiplication_6(&mults)
            };
            for i in 0..6 {
                c[check][i] = (1.0 - result[i]) / 2.0;
            }
            if var6 != 255 {
                c[check][6] = (1.0 - result[6]) / 2.0;
            }
        }
        for var in 0..174 {
            for check_id in 0..3 {
                let mut is_1 = 1.0;
                let mut is_0 = 1.0;
                for check2_id in 0..3 {
                    if check2_id != check_id {
                        let check2 = FT8_MN[var][check2_id];
                        let var_id = FT8_MNV[var][check2_id];
                        is_1 *= c[check2 as usize][var_id as usize];
                        is_0 *= 1.0 - c[check2 as usize][var_id as usize];
                    }
                }
                is_1 *= bits[var];
                is_0 *= 1.0 - bits[var];
                v[var][check_id] = is_1 / (is_0 + is_1 + 1e-12);
            }
        }
        for i in 0..174 {
            let p_0 = v[i][0];
            let p_1 = v[i][1];
            let p_2 = v[i][2];
            let is_1 = bits[i] * p_0 * p_1 * p_2;
            let is_0 = (1.0 - bits[i]) * (1.0 - p_0) * (1.0 - p_1) * (1.0 - p_2);
            output[i] = is_1 > is_0;
        }
        if ldpc_check(&output) {
            return Some(output);
        }
    }
    None
}

fn ldpc_check(codeword: &[bool; 174]) -> bool {
    for i in 0..83 {
        let mut x = false;
        for j in 0..6 {
            let var = FT8_NM[i][j];
            x ^= codeword[var as usize];
        }
        let var6 = FT8_NM[i][6];
        if var6 != 255 {
            x ^= codeword[var6 as usize];
        }
        if x {
            return false;
        }
    }
    return true;
}
