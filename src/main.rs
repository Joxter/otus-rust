#![allow(dead_code)]
#![allow(unused_imports)]

mod fibo;
mod arrays;
mod test_runner;

use crate::fibo::fibo_b;
use crate::arrays::result::run_array_tests;

fn main() {

    run_array_tests();
}
