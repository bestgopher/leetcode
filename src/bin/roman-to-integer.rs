fn main() {
    assert_eq!(3, Solution::roman_to_int("III".to_string()));
    assert_eq!(4, Solution::roman_to_int("IV".to_string()));
    assert_eq!(9, Solution::roman_to_int("IX".to_string()));
    assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let s = s.as_bytes();

        for (i, &v) in s.iter().enumerate() {
            let m = match v {
                b'I' => 1,
                b'V' => {
                    if i != 0 && s[i - 1] == b'I' {
                        3
                    } else {
                        5
                    }
                }
                b'X' => {
                    if i != 0 && s[i - 1] == b'I' {
                        8
                    } else {
                        10
                    }
                }
                b'L' => {
                    if i != 0 && s[i - 1] == b'X' {
                        30
                    } else {
                        50
                    }
                }
                b'C' => {
                    if i != 0 && s[i - 1] == b'X' {
                        80
                    } else {
                        100
                    }
                }
                b'D' => {
                    if i != 0 && s[i - 1] == b'C' {
                        300
                    } else {
                        500
                    }
                }
                b'M' => {
                    if i != 0 && s[i - 1] == b'C' {
                        800
                    } else {
                        1000
                    }
                }

                _ => 0,
            };

            result += m;
        }

        result
    }
}
