#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{:?}", Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]));
}

struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }

        let mut v = vec![];
        let (mut slow, mut fast) = (0usize, 0usize);

        while fast <= nums.len() {
            if fast == 0 {
                fast += 1;
                continue;
            }

            if fast == nums.len() || nums[fast] - nums[fast - 1] != 1 {
                if fast - 1 == slow {
                    v.push(nums[fast - 1].to_string());
                } else {
                    v.push(format!("{}->{}", nums[slow], nums[fast - 1]));
                }
                slow = fast;
            }
            fast += 1;
        }

        v
    }
}
