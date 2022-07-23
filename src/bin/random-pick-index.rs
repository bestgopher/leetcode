#![allow(dead_code, unused, unused_variables)]

fn main() {}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
struct Solution1 {
    index_map: std::collections::HashMap<i32, Vec<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution1 {
    fn new(nums: Vec<i32>) -> Self {
        let mut index_map = std::collections::HashMap::new();

        nums.into_iter().enumerate().for_each(|(index, value)| {
            index_map
                .entry(value)
                .and_modify(|y: &mut Vec<usize>| y.push(index))
                .or_insert(vec![index]);
        });

        Self { index_map }
    }

    fn pick(&self, target: i32) -> i32 {
        use rand::Rng;

        match self.index_map.get(&target) {
            Some(x) => x[rand::thread_rng().gen_range(0..x.len())] as i32,
            None => 0,
        }
    }
}

struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn pick(&self, target: i32) -> i32 {
        use rand::Rng;
        let mut n = 0;
        let mut r = 0;

        for i in 0..self.nums.len() {
            if self.nums[i] == target {
                n += 1;
                if rand::thread_rng().gen_range(0..n) == 0 {
                    r = i as i32;
                }
            }
        }

        r
    }
}
