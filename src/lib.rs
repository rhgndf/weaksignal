#![feature(test)]

extern crate test;

mod callsignmap;
mod constants;
mod ldpc;
pub mod ft8;
mod message;
mod utils;

#[cfg(test)]
mod tests {

    use crate::{callsignmap::CallsignMap, utils::{optimal_leave_one_out_multiplication_7, optimal_leave_one_out_multiplication_6}, ldpc::ldpc_decode};

    use test::Bencher;

    #[test]
    fn optimal_mult_test() {
        let mults = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let result_optimal = optimal_leave_one_out_multiplication_7(&mults);
        let mut result_naive = [1.0; 7];
        for i in 0..7 {
            for j in 0..7 {
                if i != j {
                    result_naive[i] *= mults[j];
                }
            }
        }
        for i in 0..7 {
            assert_eq!(result_naive[i], result_optimal[i]);
        }
    }


    #[test]
    fn optimal_mult_test_6() {
        let mults = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 1.0];
        let result_optimal = optimal_leave_one_out_multiplication_6(&mults);
        let mut result_naive = [1.0; 6];
        for i in 0..6 {
            for j in 0..6 {
                if i != j {
                    result_naive[i] *= mults[j];
                }
            }
        }
        for i in 0..6 {
            assert_eq!(result_naive[i], result_optimal[i]);
        }
    }

    #[bench]
    fn optimal_mult_bench(b: &mut Bencher) {
        let mults = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        b.iter(|| optimal_leave_one_out_multiplication_7(&mults));
    }

    #[bench]
    fn optimal_mult_bench_6(b: &mut Bencher) {
        let mults = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        b.iter(|| optimal_leave_one_out_multiplication_6(&mults));
    }

    #[bench]
    fn ldpc_bench(b: &mut Bencher) {
        let mut bits = [0.0; 174];
        for i in 0..174 {
            bits[i] = ((i & 1) as f32) * 0.8 + 0.1;
        }
        b.iter(|| ldpc_decode(&bits, 10));
    }
    
    #[test]
    fn callsign_hash_pj4k1abc() {
        let callsign = "PJ4/K1ABC".to_string();
        let hash = CallsignMap::hash(&callsign);
        assert_eq!(hash, 1420834);
    }

    #[test]
    fn callsign_hash_yw18fifa() {
        let callsign = "YW18FIFA".to_string();
        let hash = CallsignMap::hash(&callsign);
        assert_eq!(hash, 771524);
    }

    #[test]
    fn callsign_hash_lookup() {
        let callsign = "9V1AA".to_string();
        let mut map = CallsignMap::new();
        let hash = map.insert(&callsign);
        let lookup = map.get_22(hash);
        assert!(lookup.is_some());
        assert_eq!(lookup.unwrap().clone(), callsign);
    }
}
