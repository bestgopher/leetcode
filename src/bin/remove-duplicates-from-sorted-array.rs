fn main() {
    assert_eq!(
        5,
        Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
    )
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 || nums.len() == 1 {
            return nums.len() as i32;
        }

        let mut slow = 0usize;

        for i in 1..nums.len() {
            if nums[i] == nums[slow] {
                continue;
            } else {
                slow += 1;
                nums[slow] = nums[i];
            }
        }
        slow as i32 + 1
    }
}
