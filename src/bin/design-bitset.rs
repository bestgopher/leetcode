#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your Bitset object will be instantiated and called as such:
 * let obj = Bitset::new(size);
 * obj.fix(idx);
 * obj.unfix(idx);
 * obj.flip();
 * let ret_4: bool = obj.all();
 * let ret_5: bool = obj.one();
 * let ret_6: i32 = obj.count();
 * let ret_7: String = obj.to_string();
 */

struct Bitset {
    data: Vec<u8>,
    size: i32,
    count: i32,
}

impl Bitset {
    fn new(size: i32) -> Self {
        Self {
            data: vec![0; size as usize / 8 + 1],
            size,
            count: 0,
        }
    }

    fn set(&mut self, idx: usize, val: u8) {
        let index = idx / 8;
        let n = idx % 8;
        if val == 1 {
            let d = 0b1 << (7 - n);
            if self.data[index] != self.data[index] | d {
                self.count += 1;
                self.data[index] = self.data[index] | d;
            }
        } else {
            let d = 0b11111111 ^ (0b1 << (7 - n));
            if self.data[index] != self.data[index] & d {
                self.count -= 1;
                self.data[index] = self.data[index] & d;
            }
        }
    }

    fn fix(&mut self, idx: i32) {
        self.set(idx as usize, 1);
    }

    fn unfix(&mut self, idx: i32) {
        self.set(idx as usize, 0)
    }

    fn flip(&mut self) {
        self.count = self.size - self.count;

        for i in 0..self.data.len() {
            self.data[i] = !self.data[i];
        }
    }

    fn all(&self) -> bool {
        self.count == self.size
    }

    fn one(&self) -> bool {
        self.count > 0
    }

    fn count(&self) -> i32 {
        self.count
    }

    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.size as usize);

        'b: for i in 0..self.data.len() {
            for index in 0..8 {
                if i * 8 + index >= self.size as usize {
                    break 'b;
                }
                if self.data[i] >> (7 - index) & 0b1 == 0b1 {
                    s.push('1');
                } else {
                    s.push('0');
                }
            }
        }
        s
    }
}
