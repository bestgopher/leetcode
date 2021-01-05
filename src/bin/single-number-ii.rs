fn main() {
    assert_eq!(99, Solution::single_number1(vec![0, 1, 0, 1, 0, 1, 99]));
    assert_eq!(2147483647, Solution::single_number1(vec![43, 16, 45, 89, 45, -2147483648, 45, 2147483646, -2147483647, -2147483648, 43, 2147483647, -2147483646, -2147483648, 89, -2147483646, 89, -2147483646, -2147483647, 2147483646, -2147483647, 16, 16, 2147483646, 43]));
}

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut seen = 0;

        for &i in nums.iter() {
            r = !seen & (r ^ i);
            seen = !r & (seen ^ i);
        }

        r
    }

    /// 使用hashset
    /// 但是会溢出
    pub fn single_number1(nums: Vec<i32>) -> i32 {
        let mut h = std::collections::HashSet::new();

        for &i in nums.iter() {
            h.insert(i);
        }

        (h.iter().sum::<i32>() * 3 - nums.iter().sum::<i32>()) / 2
    }
}
