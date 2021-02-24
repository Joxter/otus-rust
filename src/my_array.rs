use std::time::SystemTime;

#[derive(Debug)]
pub struct SingleArray {
    len: usize,
    arr: Vec<i32>,
}

pub trait MyArray {
    fn new() -> Self;
    fn add(&mut self, value: i32);
    fn add_to(&mut self, value: i32, index: usize);
    fn remove(&mut self, index: usize) -> i32;
}

impl MyArray for SingleArray {
    fn new() -> SingleArray {
        SingleArray {
            arr: Vec::with_capacity(0),
            len: 0,
        }
    }

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

    pub fn add(&mut self, value: i32) {
        if self.len < self.capacity {
            self.arr[self.len] = value;
            self.len += 1;
            return;
        }

        let curr_len = self.len;
        self.capacity += self.step;
        let mut new_arr = vec![0; self.capacity];

        for i in 0..curr_len {
            new_arr[i] = self.arr[i];
        }
        new_arr[curr_len] = value;

        self.len += 1;
        self.arr = new_arr;
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

    pub fn add(&mut self, value: i32) {
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
}

pub fn test_arr<T: MyArray>(arr: &mut T, n: i32) {
    let sys_time = SystemTime::now();
    for i in 0..n {
        arr.add(i);
    }
    let difference = SystemTime::now().duration_since(sys_time).expect("lol");
    println!("time: {:?}", difference);
}

