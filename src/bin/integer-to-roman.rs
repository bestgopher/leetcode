#![allow(dead_code, unused, unused_variables)]

fn main() {
    Solution::int_to_roman(23134);
}

struct Solution;

impl Solution {
    /// 常规解法
    pub fn int_to_roman1(num: i32) -> String {
        let mut v = vec![];
        let (mut num, mut n) = (num, 1);
        while num != 0 {
            v.push((num % 10) * n);
            n *= 10;
            num /= 10;
        }

        let mut result = String::new();
        while !v.is_empty() {
            let x = v.pop().unwrap();
            match x {
                1000..=3000 => result.push_str("M".repeat((x / 1000) as usize).as_str()),
                100..=300 => result.push_str("C".repeat((x / 100) as usize).as_str()),
                400 => result.push_str("CD"),
                500 => result.push_str("D"),
                600..=800 => {
                    result.push_str("D");
                    result.push_str("C".repeat(((x - 500) / 100) as usize).as_str());
                }
                900 => result.push_str("CM"),
                10..=30 => result.push_str("X".repeat((x / 100) as usize).as_str()),
                40 => result.push_str("XL"),
                50 => result.push_str("L"),
                60..=80 => {
                    result.push_str("L");
                    result.push_str("C".repeat(((x - 500) / 100) as usize).as_str());
                }
                90 => result.push_str("XC"),
                1..=3 => result.push_str("I".repeat((x / 100) as usize).as_str()),
                4 => result.push_str("IV"),
                5 => result.push_str("V"),
                6..=8 => {
                    result.push_str("V");
                    result.push_str("I".repeat(((x - 500) / 100) as usize).as_str());
                }
                9 => result.push_str("IX"),
                _ => (),
            }
        }

        result
    }

    pub fn int_to_roman(num: i32) -> String {
        let mut stack = vec![
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
        ];
        let mut num = num;
        let mut result = String::new();

        while !stack.is_empty() {
            match stack.last() {
                Some(x) if x.0 > num => {
                    stack.pop();
                }
                Some(x) => {
                    result.push_str(x.1);
                    num -= x.0;
                }
                _ => (),
            }
        }

        result
    }
}
