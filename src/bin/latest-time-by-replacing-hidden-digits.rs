#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_time(mut time: String) -> String {
        let mut f = false;
        unsafe {
            let mut s = time.as_bytes_mut();
            for i in 0..s.len() {
                if s[i] == b'?' {
                    match i {
                        0 => {
                            if s[1] == b'?' || (s[1] >= b'0' && s[1] <= b'3') {
                                s[0] = b'2';
                            } else {
                                s[0] = b'1';
                            }
                        }
                        1 => {
                            if f {
                                s[1] = b'3';
                            } else {
                                s[1] = b'9';
                            }
                        }

                        3 => s[3] = b'5',
                        4 => s[4] = b'9',
                        _ => {}
                    }
                }

                f = s[0] == b'2';
            }
        }

        time
    }
}
