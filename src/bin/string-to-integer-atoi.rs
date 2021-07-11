fn main() {
    assert_eq!(42, Solution::my_atoi("42".to_string()));
    assert_eq!(-42, Solution::my_atoi("   -42".to_string()));
    assert_eq!(4193, Solution::my_atoi("4193 with words".to_string()));
    assert_eq!(0, Solution::my_atoi("words and 987".to_string()));
    assert_eq!(-2147483648, Solution::my_atoi("-91283472332".to_string()));
    assert_eq!(0, Solution::my_atoi(" ++1".to_string()));
    assert_eq!(2147483647, Solution::my_atoi("21474836460".to_string()));
    assert_eq!(0, Solution::my_atoi("-+12".to_string()));
}

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // 丢弃无用的前导空格
        let s = s.trim_start_matches(" ");
        // 判断是否是字母开头或者.开头
        if s.starts_with(char::is_alphabetic) || s.starts_with(".") {
            return 0;
        }

        // 结果、是否为正数、是否溢出、是否开始计算
        let (mut result, mut is_positive, mut is_overflow) = (0, true, false);

        for (i, v) in s.bytes().enumerate() {
            if i == 0 {
                if v == b'-' {
                    is_positive = false;
                } else if v == b'+' {
                    is_positive = true;
                } else {
                    result = (v - b'0') as i32;
                }
            } else {
                if v < b'0' || v > b'9' {
                    break;
                }

                let s = result.overflowing_mul(10);
                if s.1 {
                    is_overflow = true;
                    break;
                }

                let s = s.0.overflowing_add((v - b'0') as i32);
                if s.1 {
                    is_overflow = true;
                    break;
                }

                result = s.0;
            }
        }

        if is_positive {
            if is_overflow {
                std::i32::MAX
            } else {
                result
            }
        } else {
            if is_overflow {
                std::i32::MIN
            } else {
                result * -1
            }
        }
    }
}
