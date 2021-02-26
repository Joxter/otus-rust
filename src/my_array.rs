use core::fmt::Debug;
use std::time::SystemTime;

#[derive(Debug)]
pub struct SingleArray {
    len: usize,
    arr: Vec<i32>,
}

pub trait MyArray {
    fn add(&mut self, value: i32);
    fn add_to(&mut self, value: i32, index: usize);
    fn remove(&mut self, index: usize) -> i32;
}

impl SingleArray {
    pub fn new() -> SingleArray {
        SingleArray {
            arr: Vec::with_capacity(0),
            len: 0,
        }
    }
}

impl MyArray for SingleArray {
    fn add(&mut self, value: i32) {
        let mut new_arr = vec![0; self.len + 1];

        for i in 0..(self.len) {
            new_arr[i] = self.arr[i];
        }
        new_arr[self.len] = value;

        self.len = self.len + 1;
        self.arr = new_arr;
    }

    fn add_to(&mut self, value: i32, index: usize) {
        self.len = self.len + 1;
        let mut new_arr = vec![0; self.len];

        for i in 0..index {
            new_arr[i] = self.arr[i];
        }

        for i in ((index + 1)..self.len).rev() {
            new_arr[i] = self.arr[i - 1];
        }

        new_arr[index] = value;
        self.arr = new_arr;
    }

    fn remove(&mut self, index: usize) -> i32 {
        self.len = self.len - 1;
        let res = self.arr[index];
        let mut new_arr = vec![0; self.len];

        for i in 0..index {
            new_arr[i] = self.arr[i];
        }

        for i in index..self.len {
            new_arr[i] = self.arr[i + 1];
        }

        self.arr = new_arr;
        res
    }
}

#[derive(Debug)]
pub struct VectorArray {
    len: usize,
    capacity: usize,
    step: usize,
    arr: Vec<i32>,
}

impl VectorArray {
    pub fn new(step: usize) -> VectorArray {
        VectorArray {
            arr: Vec::with_capacity(0),
            len: 0,
            capacity: 0,
            step,
        }
    }
}

impl MyArray for VectorArray {
    fn add(&mut self, value: i32) {
        if self.len < self.capacity {
            self.arr[self.len] = value;
            self.len += 1;
            return;
        }

        self.capacity += self.step;
        let mut new_arr = vec![0; self.capacity];

        for i in 0..self.len {
            new_arr[i] = self.arr[i];
        }
        new_arr[self.len] = value;

        self.len += 1;
        self.arr = new_arr;
    }

    fn add_to(&mut self, value: i32, index: usize) {
        if self.len == self.capacity {
            self.capacity += self.step;
        }
        self.len += 1;

        let mut new_arr = vec![0; self.capacity];

        for i in 0..index {
            new_arr[i] = self.arr[i];
        }

        for i in ((index + 1)..self.len).rev() {
            new_arr[i] = self.arr[i - 1];
        }
        new_arr[index] = value;

        self.arr = new_arr;
    }

    fn remove(&mut self, index: usize) -> i32 {
        self.len -= 1;

        if self.capacity - self.len >= self.step {
            self.capacity -= self.step;
        }

        let res = self.arr[index];
        let mut new_arr = vec![0; self.capacity];

        for i in 0..index {
            new_arr[i] = self.arr[i];
        }

        for i in index..self.len {
            new_arr[i] = self.arr[i + 1];
        }

        self.arr = new_arr;
        res
    }
}

#[derive(Debug)]
pub struct FactorArray {
    len: usize,
    capacity: usize,
    arr: Vec<i32>,
}

impl FactorArray {
    pub fn new() -> FactorArray {
        FactorArray {
            arr: Vec::with_capacity(0),
            len: 0,
            capacity: 0,
        }
    }
}

impl MyArray for FactorArray {
    fn add(&mut self, value: i32) {
        if self.len < self.capacity {
            self.arr[self.len] = value;
            self.len += 1;
            return;
        }

        let curr_len = self.len;
        if self.capacity == 0 {
            self.capacity = 1
        } else {
            self.capacity *= 2;
        }
        let mut new_arr = vec![0; self.capacity];

        for i in 0..curr_len {
            new_arr[i] = self.arr[i];
        }
        new_arr[curr_len] = value;

        self.len += 1;
        self.arr = new_arr;
    }

    fn add_to(&mut self, value: i32, index: usize) {
        if self.len == self.capacity {
            self.capacity = self.capacity * 2 + 1;
        }
        self.len += 1;

        let mut new_arr = vec![0; self.capacity];

        for i in 0..index {
            new_arr[i] = self.arr[i];
        }

        for i in ((index + 1)..self.len).rev() {
            new_arr[i] = self.arr[i - 1];
        }
        new_arr[index] = value;

        self.arr = new_arr;
    }

    fn remove(&mut self, index: usize) -> i32 {
        self.len -= 1;

        if self.len * 2 < self.capacity {
            self.capacity /= 2;
        }

        let res = self.arr[index];
        let mut new_arr = vec![0; self.capacity];

        for i in 0..index {
            new_arr[i] = self.arr[i];
        }

        for i in index..self.len {
            new_arr[i] = self.arr[i + 1];
        }

        self.arr = new_arr;
        res
    }
}

#[derive(Debug)]
pub struct PureArray {
    len: usize,
    arr: Vec<i32>,
}

impl PureArray {
    pub fn new() -> PureArray {
        PureArray {
            arr: Vec::with_capacity(0),
            len: 0,
        }
    }
}

impl MyArray for PureArray {
    fn add(&mut self, value: i32) {
        self.len += 1;
        self.arr.push(value);
    }

    fn add_to(&mut self, value: i32, index: usize) {
        self.len += 1;
        self.arr.insert(index, value);
    }

    fn remove(&mut self, index: usize) -> i32 {
        self.arr.remove(index)
    }
}

fn test_arr<T: MyArray>(title: &str, arr: &mut T, n: i32) {
    let sys_time = SystemTime::now();
    for i in 0..n {
        arr.add(i);
    }
    let difference = SystemTime::now().duration_since(sys_time).unwrap();
    println!(
        "Test: \"{}\" N:{} time: {:.4} sec",
        title,
        n,
        difference.as_secs_f32()
    );
}

fn test_array<T: MyArray + std::fmt::Debug>(my_arr: &mut T) {
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
}

/*
Test: "SingleArray"  N:1000 time: 0.0386 sec
Test: "SingleArray" N:10000 time: 3.8576 sec
Test: "SingleArray" N:20000 time: 15.5518 sec

Test: "VectorArray::new(10)"     N:1000 time: 0.0039 sec
Test: "VectorArray::new(10)"    N:10000 time: 0.3795 sec
Test: "VectorArray::new(10)"    N:20000 time: 1.4855 sec
Test: "VectorArray::new(10)"    N:50000 time: 9.5125 sec
Test: "VectorArray::new(100)"    N:1000 time: 0.0004 sec
Test: "VectorArray::new(100)"   N:10000 time: 0.0367 sec
Test: "VectorArray::new(100)"   N:20000 time: 0.1579 sec
Test: "VectorArray::new(100)"   N:50000 time: 0.9461 sec
Test: "VectorArray::new(100)"  N:100000 time: 3.7942 sec
Test: "VectorArray::new(1000)" N:100000 time: 0.3959 sec
Test: "VectorArray::new(1000)" N:500000 time: 9.5184 sec

Test: "FactorArray::new()"     N:1000 time: 0.0001 sec
Test: "FactorArray::new()"    N:10000 time: 0.0018 sec
Test: "FactorArray::new()"   N:100000 time: 0.0152 sec
Test: "FactorArray::new()"  N:1000000 time: 0.1353 sec
Test: "FactorArray::new()" N:10000000 time: 1.8687 sec

Test: "PureArray::new()"     N:1000 time: 0.0001 sec
Test: "PureArray::new()"    N:10000 time: 0.0005 sec
Test: "PureArray::new()"   N:100000 time: 0.0049 sec
Test: "PureArray::new()"  N:1000000 time: 0.0514 sec
Test: "PureArray::new()" N:10000000 time: 0.5432 sec
*/
pub fn run_array_tests() {
    // test_array(&mut SingleArray::new());
    // test_array(&mut VectorArray::new(3));
    // test_array(&mut FactorArray::new());

    // test_arr("SingleArray", &mut SingleArray::new(), 1_000);
    // test_arr("SingleArray", &mut SingleArray::new(), 10_000);
    // test_arr("SingleArray", &mut SingleArray::new(), 20_000);

    // test_arr("VectorArray::new(10)", &mut VectorArray::new(10), 1_000);
    // test_arr("VectorArray::new(10)", &mut VectorArray::new(10), 10_000);
    // test_arr("VectorArray::new(10)", &mut VectorArray::new(10), 20_000);
    // test_arr("VectorArray::new(10)", &mut VectorArray::new(10), 50_000);
    // test_arr("VectorArray::new(100)", &mut VectorArray::new(100), 1_000);
    // test_arr("VectorArray::new(100)", &mut VectorArray::new(100), 10_000);
    // test_arr("VectorArray::new(100)", &mut VectorArray::new(100), 20_000);
    // test_arr("VectorArray::new(100)", &mut VectorArray::new(100), 50_000);
    // test_arr("VectorArray::new(100)", &mut VectorArray::new(100), 100_000);
    // test_arr("VectorArray::new(1000)", &mut VectorArray::new(1000), 100_000);
    // test_arr("VectorArray::new(1000)", &mut VectorArray::new(1000), 500_000);

    // test_arr("FactorArray::new()", &mut FactorArray::new(), 1_000);
    // test_arr("FactorArray::new()", &mut FactorArray::new(), 10_000);
    // test_arr("FactorArray::new()", &mut FactorArray::new(), 100_000);
    // test_arr("FactorArray::new()", &mut FactorArray::new(), 1_000_000);
    // test_arr("FactorArray::new()", &mut FactorArray::new(), 10_000_000);

    // test_arr("PureArray::new()", &mut PureArray::new(), 1_000);
    // test_arr("PureArray::new()", &mut PureArray::new(), 10_000);
    // test_arr("PureArray::new()", &mut PureArray::new(), 100_000);
    // test_arr("PureArray::new()", &mut PureArray::new(), 1_000_000);
    // test_arr("PureArray::new()", &mut PureArray::new(), 10_000_000);
}
