fn main() {
    let mut v = vec![2, 0, 2, 1, 1, 1, 1, 1, 0, 2, 2, 2, 2, 0, 0, 2, 1, 0];
    Solution::sort_colors(&mut v);
}

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut p1, mut p2, mut cursor) = (0, 0, 0);

        while cursor < nums.len() {
            if nums[cursor] == 0 {
                nums[cursor] = nums[p1];
                nums[p1] = 0;
                p1 += 1;
            }

            if nums[cursor] == 1 {
                nums[cursor] = nums[p2];
                nums[p2] = 1;
                p2 += 1;
            }
            cursor += 1;

            if cursor == 1 {
                cursor -= 1;
            }

            println!("{:?}", nums);
        }
    }
}
