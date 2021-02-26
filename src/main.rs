#![allow(dead_code)]
#![allow(unused_imports)]

mod fibo;
mod my_array;
mod test_runner;

use crate::fibo::fibo_b;
use crate::my_array::{run_array_tests};

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

    run_array_tests();
}
