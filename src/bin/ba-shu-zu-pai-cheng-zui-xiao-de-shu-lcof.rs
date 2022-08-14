#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    println!("{}", Solution::min_number(vec![10, 2]));
    println!("{}", Solution::min_number(vec![3, 30, 34, 5, 9]));
}

struct Solution;

impl Solution {
    /// 如果 x + y > y + x, 则 x > y
    /// 如果 x + y < y + x, 则 x < y
    pub fn min_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<String> = nums.into_iter().map(|x| x.to_string()).collect();
        nums.sort_by(|x, y| {
            let mut s = String::with_capacity(x.len() + y.len());
            s.push_str(&x);
            s.push_str(&y);

            let mut s1 = String::with_capacity(x.len() + y.len());
            s1.push_str(&y);
            s1.push_str(&x);

            s.cmp(&s1)
        });
        nums.join("")
    }
}
