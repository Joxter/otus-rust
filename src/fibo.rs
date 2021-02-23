use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

pub fn fibo_b(n: usize) -> BigUint {
    if n == 1 {
        return One::one();
    }
    if n == 0 {
        return Zero::zero();
    }

    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}
