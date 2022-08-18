#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut v = [0; 32];

        for mut i in nums {
            let mut index = 0;
            while i > 0 {
                if i & 1 == 1 {
                    v[index] += 1;
                }
                index += 1;
                i >>= 1;
            }
        }

        let mut result = 0;

        for (index, &value) in v.iter().enumerate() {
            if value % 3 != 0 {
                result |= (1 << index);
            }
        }

        result
    }
}
