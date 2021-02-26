pub mod result;

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
