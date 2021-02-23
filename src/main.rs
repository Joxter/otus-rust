mod fibo;
mod my_array;
mod test_runner;

use crate::fibo::fibo_b;
use crate::test_runner::run_test;

fn main() {
    run_test("./src/4.Fibo", |input| {
        let result = fibo_b(input.trim().parse().unwrap());
        result.to_string()
    });
}
