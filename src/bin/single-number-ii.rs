fn main() {}

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
}
