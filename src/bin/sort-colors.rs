fn main() {
    let mut v = vec![2, 0, 2, 1, 1, 1, 1, 1, 0, 2, 2, 2, 2, 0, 0, 2, 1, 0];
    Solution::sort_colors(&mut v);

    let mut v = vec![1, 0];
    Solution::sort_colors(&mut v);
}

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut p1, mut p2, mut cursor) = (0, 0, 0);

        while cursor < nums.len() {
            if nums[cursor] == 0 {
                nums.swap(cursor, p1);
                if p1 < p2 {
                    nums.swap(cursor, p2);
                }
                p2 += 1;
                p1 += 1;
            } else if nums[cursor] == 1 {
                nums.swap(cursor, p2);
                p2 += 1;
            }
            cursor += 1;
        }
    }
}
