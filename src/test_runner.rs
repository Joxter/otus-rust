use std::fs;
use std::path::Path;
use std::time::SystemTime;

pub fn run_test<F>(path: &str, cb: F)
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

pub fn run_timer<F>(cb: F)
where
    F: Fn() -> (),
{
    let sys_time = SystemTime::now();
    cb();
    let difference = SystemTime::now().duration_since(sys_time).expect("lol");
    println!("time: {:?}", difference);
}
