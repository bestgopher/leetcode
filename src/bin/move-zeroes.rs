fn main() {
    let mut v = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut v);
    println!("{:?}", v);
}

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut i, mut j) = (0, 0);

        while j < nums.len() {
            match (nums[i], nums[j]) {
                (0, 0) => {
                    j += 1;
                }
                (_, 0) => {
                    j += 1;
                }
                (0, _) => {
                    nums.swap(i, j);
                    i += 1;
                }
                _ => {
                    i += 1;
                    j += 1;
                }
            }
        }
    }
}
