macro_rules! slice_to_val {
    ($name:ident, $type:ident) => {
        pub(crate) fn $name<const SIZE: usize>(bits: &[bool; SIZE]) -> $type {
            bits.iter()
                .fold(0, |acc, &x| (acc << 1) | if x { 1 } else { 0 })
        }
    };
}

slice_to_val!(bool_slice_to_u128, u128);
slice_to_val!(bool_slice_to_u64, u64);
slice_to_val!(bool_slice_to_u32, u32);
slice_to_val!(bool_slice_to_u16, u16);
slice_to_val!(bool_slice_to_u8, u8);

pub(crate) fn char_lookup(x: usize, charset: &str) -> char {
    if x > charset.len() {
        return '?';
    }
    return charset.as_bytes()[x] as char;
}

pub(crate) fn optimal_leave_one_out_multiplication_7(mults: &[f32; 7]) -> [f32; 7] {
    let mut ret = [0.0; 7];
    let suffix_6 = mults[6];
    let suffix_5 = mults[5] * suffix_6;
    let suffix_4 = mults[4] * suffix_5;
    let suffix_3 = mults[3] * suffix_4;
    let suffix_2 = mults[2] * suffix_3;
    let suffix_1 = mults[1] * suffix_2;

    let mut mult = mults[0];
    ret[0] = suffix_1;
    ret[1] = mult * suffix_2;
    mult *= mults[1];
    ret[2] = mult * suffix_3;
    mult *= mults[2];
    ret[3] = mult * suffix_4;
    mult *= mults[3];
    ret[4] = mult * suffix_5;
    mult *= mults[4];
    ret[5] = mult * suffix_6;
    mult *= mults[5];
    ret[6] = mult;
    ret
}

pub(crate) fn optimal_leave_one_out_multiplication_6(mults: &[f32; 7]) -> [f32; 7] {
    let mut ret = [0.0; 7];
    let suffix_5 = mults[5];
    let suffix_4 = mults[4] * suffix_5;
    let suffix_3 = mults[3] * suffix_4;
    let suffix_2 = mults[2] * suffix_3;
    let suffix_1 = mults[1] * suffix_2;

    let mut mult = mults[0];
    ret[0] = suffix_1;
    ret[1] = mult * suffix_2;
    mult *= mults[1];
    ret[2] = mult * suffix_3;
    mult *= mults[2];
    ret[3] = mult * suffix_4;
    mult *= mults[3];
    ret[4] = mult * suffix_5;
    mult *= mults[4];
    ret[5] = mult;
    ret
}