#![allow(dead_code)]
#![allow(unused_imports)]

mod fibo;
mod my_array;
mod test_runner;

use crate::fibo::fibo_b;
use crate::my_array::run_array_tests;

fn main() {
    // run_test("./src/4.Fibo", |input| {
    //     let result = fibo_b(input.trim().parse().unwrap());
    //     result.to_string()
    // });

    run_array_tests();
}
