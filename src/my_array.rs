#[derive(Debug)]
pub struct SingleArray {
    len: usize,
    arr: Vec<i32>,
}

// todo implement method: T remove(int index);
impl SingleArray {
    pub fn new() -> SingleArray {
        SingleArray {
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
