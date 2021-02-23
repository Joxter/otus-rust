#[derive(Debug)]
pub struct MyDumpArray {
    len: usize,
    arr: Vec<i32>,
}

impl MyDumpArray {
    pub fn new() -> MyDumpArray {
        MyDumpArray {
            arr: Vec::with_capacity(0),
            len: 0,
        }
    }

    pub fn add(&mut self, value: i32) {
        let curr_len = self.len;
        let new_len = curr_len + 1;
        let mut new_arr = vec![0; new_len];

        for i in 0..curr_len {
            new_arr[i] = self.arr[i];
        }
        new_arr[curr_len] = value;

        self.len = new_len;
        self.arr = new_arr;
    }
}

#[derive(Debug)]
pub struct MyAddArray {
    len: usize,
    capacity: usize,
    arr: Vec<i32>,
}

const STEP: usize = 100;

impl MyAddArray {
    pub fn new() -> MyAddArray {
        MyAddArray {
            arr: Vec::with_capacity(0),
            len: 0,
            capacity: 0,
        }
    }

    pub fn add(&mut self, value: i32) {
        if self.len < self.capacity {
            self.len += 1;
            self.arr.push(value);
            return;
        }

        let curr_len = self.len;
        self.capacity += STEP;
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
pub struct MyMulArray {
    len: usize,
    capacity: usize,
    arr: Vec<i32>,
}

impl MyMulArray {
    pub fn new() -> MyMulArray {
        MyMulArray {
            arr: Vec::with_capacity(0),
            len: 0,
            capacity: 0,
        }
    }

    pub fn add(&mut self, value: i32) {
        if self.len < self.capacity {
            self.len += 1;
            self.arr.push(value);
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
