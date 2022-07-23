#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{}", Solution::fraction_to_decimal(1, 2));
    println!("{}", Solution::fraction_to_decimal(2, 1));
    println!("{}", Solution::fraction_to_decimal(4, 333));
    println!("{}", Solution::fraction_to_decimal(1, 5));
    println!("{}", Solution::fraction_to_decimal(424231, 5));
    println!("{}", Solution::fraction_to_decimal(1000000, 3));
    println!("{}", Solution::fraction_to_decimal(1, 3));
    println!("{}", Solution::fraction_to_decimal(1, 6));
    println!("{}", Solution::fraction_to_decimal(1, 333));
    println!("{}", Solution::fraction_to_decimal(1, 17));
    println!("{}", Solution::fraction_to_decimal(-50, 8));
    println!("{}", Solution::fraction_to_decimal(-50, -8));
    println!("{}", Solution::fraction_to_decimal(50, -8));
    println!("{}", Solution::fraction_to_decimal(-1, -2147483648));
}

struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        // 用于保存出现过的余数
        let mut m = std::collections::HashMap::<i64, usize>::new();
        let (mut numerator, mut denominator) = (numerator as i64, denominator as i64);
        let mut flag = false; // false为当前为整数部分，true为小数部分
        let mut n = 0;
        let (mut r1, mut r2) = (String::new(), String::new()); // r1整数，r2小数

        if numerator < 0 && denominator > 0 {
            r1.push('-');
            numerator *= -1;
        } else if numerator > 0 && denominator < 0 {
            r1.push('-');
            denominator *= -1;
        } else if numerator < 0 && denominator < 0 {
            denominator *= -1;
            numerator *= -1;
        }

        loop {
            if !flag {
                r1.push_str(&(numerator / denominator).to_string());
                numerator %= denominator;
                if numerator < denominator {
                    flag = true;
                    numerator *= 10;
                }
            } else {
                n += 1;
                if let Some(&s) = m.get(&numerator) {
                    r2 = format!("{}({})", &r2[..s - 1], &r2[s - 1..]);
                    break;
                }
                m.insert(numerator, n);
                if numerator > denominator {
                    r2.push_str(&(numerator / denominator).to_string());
                    numerator %= denominator;
                } else {
                    r2.push('0');
                }
                numerator *= 10;
            }

            if numerator == 0 {
                break;
            }
        }
        if r2 != "" {
            format!("{}.{}", r1, r2)
        } else {
            r1
        }
    }
}
