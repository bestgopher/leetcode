fn main() {}

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sums = Vec::with_capacity(nums.len());
        sums.push(nums[0]);

        for i in 1..nums.len() {
            sums.push(nums[i] + sums[i - 1]);
        }

        let (mut min, mut sum) = (sums[0], sums[0]);

        for &i in sums.iter().skip(1) {
            sum = sum.max(i.max(i - min));
            min = min.min(i);
        }

        sum
    }
}
