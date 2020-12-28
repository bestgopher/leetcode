fn main() {}

struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = Vec::with_capacity(nums.len());

        for (i, v) in nums.iter().enumerate() {
            if i == 0 {
                sum.push(*v);
            } else {
                sum.push(sum[i - 1] + *v);
            }
        }

        sum
    }
}