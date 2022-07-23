#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
struct NumArray {
    sums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = Vec::with_capacity(nums.len());

        for i in 0..=nums.len() {
            if i == 0 {
                sums.push(0);
            } else {
                sums.push(nums[i] + sums[i - 1]);
            }
        }

        Self { sums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.sums[right as usize]
        } else {
            self.sums[right as usize] - self.sums[left as usize - 1usize]
        }
    }
}
