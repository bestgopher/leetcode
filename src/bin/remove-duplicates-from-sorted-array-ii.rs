#![allow(dead_code, unused, unused_variables)]

fn main() {
    let mut v = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    let s = Solution::remove_duplicates(&mut v);
    println!("{:?}", &v[..s as usize]);
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut index1, mut num) = (0, 0);

        for i in 0..nums.len() {
            if i == 0 || nums[i] == nums[i - 1] {
                num += 1;
            } else {
                num = 1;
            }

            if num <= 2 {
                nums[index1] = nums[i];
                index1 += 1;
                continue;
            }
        }

        index1 as i32
    }
}
