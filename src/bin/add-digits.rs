fn main() {
    println!("{}", Solution::add_digits(38));
    println!("{}", Solution::add_digits(3143243));
}

struct Solution;

impl Solution {
    // 超出时间限制
    pub fn add_digits1(num: i32) -> i32 {
        if num < 10 {
            return num;
        }

        let mut num = num;
        let mut s = 0;
        while num > 10 {
            s += num % 10;
            num /= 10;
        }

        Self::add_digits(s + num)
    }

    /// 能够被9整除的整数，各位上的数字加起来也必然能被9整除，所以，连续累加起来，最终必然就是9。
    /// 不能被9整除的整数，各位上的数字加起来，结果对9取模，和初始数对9取摸，是一样的，所以，连续累加起来，最终必然就是初始数对9取摸。
    pub fn add_digits(num: i32) -> i32 {
        if num % 9 == 0 {
            if num == 0 {
                0
            } else {
                9
            }
        } else {
            num % 9
        }
    }
}
