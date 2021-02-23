pub struct MyArray {
    arr: Vec<i32>,
}

impl MyArray {
    fn new() -> MyArray {
        MyArray { arr: Vec::new() }
    }

    fn add(&mut self, value: i32) {
        let curr_len = self.arr.len();
        let mut new_arr = Vec::with_capacity(curr_len + 1);

        for i in 0..curr_len {
            new_arr[i] = self.arr[i];
        }
        new_arr[curr_len + 1] = value;

        self.arr = new_arr;
    }
}
