#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * let obj = FrequencyTracker::new();
 * obj.add(number);
 * obj.delete_one(number);
 * let ret_3: bool = obj.has_frequency(frequency);
 */

struct FrequencyTracker {
    /// 每个数字对应的次数
    s1: std::collections::HashMap<i32, i32>,
    /// 每个次数对应的数字个数
    s2: std::collections::HashMap<i32, i32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        Self {
            s1: Default::default(),
            s2: Default::default(),
        }
    }

    fn add(&mut self, number: i32) {
        self.s1.entry(number).and_modify(|x| *x += 1).or_insert(1);
        let n = self.s1[&number];

        self.s2.entry(n).and_modify(|x| *x += 1).or_insert(1);
        self.remove_s2_or_delete(n - 1);
    }

    fn remove_s2_or_delete(&mut self, times: i32) {
        self.s2.entry(times).and_modify(|x| *x -= 1);
        let &x = self.s2.get(&times).unwrap_or(&0);
        if x == 0 {
            self.s2.remove(&times);
        }
    }

    fn delete_one(&mut self, number: i32) {
        self.s1.entry(number).and_modify(|x| *x -= 1);
        match self.s1.get(&number) {
            Some(&x) => {
                if x == 0 {
                    self.s1.remove(&number);
                }

                self.s2.entry(x).and_modify(|x| *x += 1).or_insert(1);
                self.remove_s2_or_delete(x + 1);
            }
            _ => {}
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.s2.get(&frequency).is_some()
    }
}
