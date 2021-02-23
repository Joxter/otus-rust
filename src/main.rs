#![allow(dead_code)]
#![allow(unused_imports)]

mod fibo;
mod my_array;
mod test_runner;

use crate::fibo::fibo_b;
use crate::test_runner::{run_test, run_timer};
use crate::my_array::{MyDumpArray, MyAddArray, MyMulArray};

fn main() {
    // run_test("./src/4.Fibo", |input| {
    //     let result = fibo_b(input.trim().parse().unwrap());
    //     result.to_string()
    // });

/*    run_timer(|| {
        let mut my_arr = MyDumpArray::new();
        for i in 0..10_000 {
            // 1_000 ~36ms
            // 10_000 ~3.74s
            // 100_000 ~371.1s
            my_arr.add(i);
        }
    })
*/
/*    run_timer(|| {
        let mut my_arr = MyAddArray::new();
        for i in 0..200_000 {
            // step 10
            // 1_000 ~3.6ms
            // 10_000 ~330.34ms
            // 100_000 ~37.64s

            // step 100
            // 1_000 ~0.3ms
            // 10_000 ~37.78ms
            // 100_000 ~3.95s
            // 200_000 ~15.72s
            my_arr.add(i);
        }
    });
*/
/*    run_timer(|| {
        let mut my_arr = MyMulArray::new();
        for i in 0..1_000_000 {
            // 1_000 ~0.1ms
            // 10_000 ~1.72ms
            // 100_000 ~15.95ms
            // 1_000_000 ~140.89ms
            // 10_000_000 ~136.38ms
            my_arr.add(i);
        }
    });
*/
/*    run_timer(|| {
        let mut my_arr = vec![];
        for i in 0..10_000_000 {
            // 100_000 ~5.95ms
            // 1_000_000 ~50.89ms
            // 10_000_000 ~522.41ms
            my_arr.push(i);
        }
    });
*/
}
