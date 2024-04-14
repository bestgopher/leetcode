#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

struct MyHashSet {
    data: Box<Vec<u8>>,
}

impl MyHashSet {
    const S: u8 = 0b10000000;

    fn new() -> Self {
        Self {
            data: Box::new(vec![0; 10usize.pow(6u32) / 8 + 1]),
        }
    }

    fn add(&mut self, key: i32) {
        let index1 = key as usize / 8;
        let index2 = key as usize % 8;

        self.data[index1] |= Self::S >> index2;
    }

    fn remove(&mut self, key: i32) {
        let index1 = key as usize / 8;
        let index2 = key as usize % 8;

        self.data[index1] &= !(Self::S >> index2);
    }

    fn contains(&self, key: i32) -> bool {
        let index1 = key as usize / 8;
        let index2 = key as usize % 8;

        self.data[index1] & Self::S >> index2 != 0
    }
}
