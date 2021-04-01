fn main() {}

struct Solution;

impl Solution {
    /// 暴力破解法
    pub fn three_sum_closest1(nums: Vec<i32>, target: i32) -> i32 {
        let mut sum: i32 = nums[0..3].iter().sum();

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    let s = nums[i] + nums[j] + nums[k];
                    if s == target {
                        return s;
                    }
                    sum = if (sum - target).abs() > (s - target).abs() { s } else { sum };
                }
            }
        }

        sum
    }

    /// 排序后，和是递增的
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut sum: i32 = nums[..3].iter().sum();

        // 因为排序后递增，所以前三个数的和是最小的，当前三个数的和都大于target时，其他数的和肯定大于target
        if sum > target {
            return sum;
        }

        for i in 0..nums.len() {
            let (mut left, mut right) = (i + 1, nums.len() - 1);

            while left < right {
                let s = nums[i] + nums[left] + nums[right];
                if s == target {
                    return s;
                } else if s > target {
                    right -= 1;
                } else {
                    left += 1;
                }

                sum = if (s - target).abs() > (sum - target).abs() { sum } else { s };
            }
        }

        sum
    }
}
