fn main() {}

struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        for i in 1..nums.len() {

            if nums[i] > nums[i-1] {
                start = i as i32;
            }

            if i + 1 < nums.len() && nums[i] > nums[i+1] {
                break;
            }
        }

        start
    }
}
