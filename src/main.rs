use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::fs;
use std::mem::replace;
use std::path::Path;
use std::time::SystemTime;

fn main() {
    run_test("./src/4.Fibo", |input| {
        let result = fibo_b(input.trim().parse().unwrap());
        result.to_string()
    });
}

fn run_test<F>(path: &str, cb: F)
where
    F: Fn(String) -> String,
{
    println!("Folder {}", path);

    for i in 0.. {
        let path_in = format!("{}/test.{}.in", path, i);
        let path_out = format!("{}/test.{}.out", path, i);

        if !Path::new(&path_in).exists() {
            break;
        }

        let input = fs::read_to_string(&path_in).expect("lol");
        let expected = fs::read_to_string(&path_out).expect("lol");

        let sys_time = SystemTime::now();
        let result = cb(input);
        let difference = SystemTime::now().duration_since(sys_time).expect("lol");

        if result == expected.trim() {
            println!("Test #{:<2}  [OK]   time: {:?}", i, difference);
        } else {
            println!("Test #{:<2} [ERROR] time: {:?}", i, difference);
            println!("result:   {}", result);
            println!("expected: {}", expected);
        }
    }
}

fn fibo_b(n: usize) -> BigUint {
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
