#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut count = [0; 26];

        for i in s.as_bytes() {
            count[(*i - b'a') as usize] += 1;
        }

        let mut result = vec![];
        'L: loop {
            for i in (0..26).rev() {
                if count[i] == 0 {
                    if i == 0 {
                        break 'L;
                    }
                    continue;
                }

                if count[i] <= repeat_limit {
                    for j in 0..count[i] {
                        result.push(i as u8 + b'a');
                    }
                    count[i] = 0;
                    continue 'L;
                } else {
                    for j in 0..repeat_limit {
                        result.push(i as u8 + b'a');
                    }
                    count[i] -= repeat_limit;
                    let mut flag = false;
                    for j in (0..i).rev() {
                        if count[j] != 0 {
                            result.push(j as u8 + b'a');
                            count[j] -= 1;
                            flag = true;
                            continue 'L;
                        }
                    }

                    if !flag {
                        break 'L;
                    }
                }
            }
        }

        unsafe { String::from_utf8_unchecked(result) }
    }
}
