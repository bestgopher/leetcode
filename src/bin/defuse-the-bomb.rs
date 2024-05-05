#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
    assert_eq!(Solution::decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
    assert_eq!(Solution::decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
}

struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = vec![0; code.len()];
        let mut code = code;

        for i in 1..code.len() {
            code[i] += code[i - 1];
        }

        for i in 0..code.len() {
            match k {
                0 => result[i] = 0,
                _x if _x > 0 => {
                    if (i + k as usize) < code.len() {
                        result[i] = code[i + k as usize] - code[i];
                    } else {
                        result[i] += code[code.len() - 1] - code[i];
                        result[i] += code[k as usize - (code.len() - 1 - i) - 1];
                    }
                }
                _ => {
                    let k = -k;
                    if i >= k as usize + 1 {
                        result[i] = code[i - 1] - code[i - k as usize - 1];
                    } else {
                        result[i] += code[code.len() - 1] - code[code.len() - 1 - (k as usize - i)];
                        if i != 0 {
                            result[i] += code[i - 1];
                        }
                    }
                }
            }
        }

        result
    }
}
