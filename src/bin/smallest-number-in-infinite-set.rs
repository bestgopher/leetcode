#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

struct SmallestInfiniteSet {
    s: std::collections::BTreeSet<i32>,
    latest: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            s: std::collections::BTreeSet::new(),
            latest: 1,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(&x) = self.s.iter().next() {
            if x < self.latest {
                self.s.remove(&x);
                return x;
            }
        }

        self.latest += 1;
        self.latest - 1
    }

    fn add_back(&mut self, num: i32) {
        if num >= self.latest {
            return;
        }

        if num == self.latest - 1 {
            self.latest -= 1;
            return;
        }

        self.s.insert(num);
    }
}
