#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{:?}", Solution::plus_one(vec![1]));
    println!("{:?}", Solution::plus_one(vec![9]));
    println!("{:?}", Solution::plus_one(vec![9, 9]));
    println!("{:?}", Solution::plus_one(vec![1, 9]));
    println!("{:?}", Solution::plus_one(vec![1, 9, 9]));
    println!("{:?}", Solution::plus_one(vec![1, 2, 3]));
    println!("{:?}", Solution::plus_one(vec![4, 3, 2, 1]));
    println!("{:?}", Solution::plus_one(vec![0]));
}

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let l = digits.len() - 1;
        // 当最后一位不为9时
        if digits[l] != 9 {
            digits[l] += 1;
        } else {
            for i in (0..=l).rev() {
                if digits[i] != 9 {
                    digits[i] += 1;
                    break;
                }
                digits[i] = 0;
                if i == 0 {
                    digits[i] += 1;
                    digits.push(0);
                }
            }
        }
        digits
    }
}
