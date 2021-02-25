#![allow(dead_code)]
#![allow(unused_imports)]

mod fibo;
mod my_array;
mod test_runner;

use crate::fibo::fibo_b;
use crate::my_array::{FactorArray, MyArray, SingleArray, VectorArray, test_arr};
use crate::test_runner::{run_test, run_timer};

fn main() {
    // run_test("./src/4.Fibo", |input| {
    //     let result = fibo_b(input.trim().parse().unwrap());
    //     result.to_string()
    // });

/*       let mut my_arr = VectorArray::new(3);
        my_arr.add(1);
        my_arr.add(2);
        my_arr.add(3);
        my_arr.add(4);
        my_arr.add(5);

        println!("arr: {:?}", my_arr);

        my_arr.add_to(10, 2);
        println!("arr: {:?}", my_arr);

        let removed = my_arr.remove(3);
        println!("arr: {:?} -------- {:?}", my_arr, removed);
*/

    // test_arr(&mut SingleArray::new(), 1_000); // ~0.03 s
    // test_arr(&mut SingleArray::new(), 10_000); // ~3.87 s
    // test_arr(&mut SingleArray::new(), 20_000); // ~15.5 s

    // test_arr(&mut VectorArray::new(10), 1_000); // ~0.03 s
    // test_arr(&mut VectorArray::new(10), 10_000); // ~0.362 s
    // test_arr(&mut VectorArray::new(10), 20_000); // ~1.54 s
    // test_arr(&mut VectorArray::new(10), 50_000); // ~9.24 s

    // test_arr(&mut VectorArray::new(100), 1_000); // ~0.00 s
    // test_arr(&mut VectorArray::new(100), 10_000); // ~0.03 s
    // test_arr(&mut VectorArray::new(100), 20_000); // ~0.14 s
    // test_arr(&mut VectorArray::new(100), 50_000); // ~0.92 s
    // test_arr(&mut VectorArray::new(100), 100_000); // ~3.74 s
    // test_arr(&mut VectorArray::new(1000), 100_000); // ~0.37 s
    // test_arr(&mut VectorArray::new(1000), 500_000); // ~9.38 s

    /*    run_timer(|| {
            let mut my_arr = FactorArray::new();
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
