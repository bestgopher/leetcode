#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_repeat_number1(nums: Vec<i32>) -> i32 {
        let mut count = vec![0; nums.len()];
        for i in nums {
            if count[i as usize] == 1 {
                return i;
            } else {
                count[i as usize] = 1;
            }
        }

        -1
    }

    pub fn find_repeat_number(mut nums: Vec<i32>) -> i32 {
        'out: for i in 0..nums.len() {
            loop {
                let x = nums[i] as usize;
                if i == x {
                    continue 'out;
                }

                if nums[x] == x as i32 {
                    return nums[x];
                }

                nums.swap(i, x);
            }
        }

        -1
    }
}
