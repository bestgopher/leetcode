#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let (mut a, mut b, mut c) = (None, None, None);
        for i in nums {
            if a.is_none() || Some(i) > a {
                c = b;
                b = a;
                a = Some(i);
            } else if (b.is_none() || Some(i) > b) && Some(i) != a {
                c = b;
                b = Some(i);
            } else if (c.is_none() || Some(i) > c) && Some(i) != b && Some(i) != a {
                c = Some(i);
            }
        }

        c.unwrap_or(a.unwrap())
    }
}
