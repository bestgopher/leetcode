#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert!(Solution::valid_utf8(vec![197, 130, 1]));
    assert!(!Solution::valid_utf8(vec![235, 140, 4]));
    assert!(Solution::valid_utf8(vec![197, 130, 1]));
    assert!(!Solution::valid_utf8(vec![248, 130, 130, 130]));
}

struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut index = 0;
        while index < data.len() {
            let n = if data[index] & 248 == 240 {
                4
            } else if data[index] & 240 == 224 {
                3
            } else if data[index] & 224 == 192 {
                2
            } else if data[index] & 128 == 0 {
                1
            } else {
                return false;
            };

            if index + n > data.len() || !Self::check(&data[index..index + n]) {
                return false;
            }

            index += n;
        }

        index == data.len()
    }

    /// 检查除第一位的后几位是否都是10开头的
    fn check(data: &[i32]) -> bool {
        for &i in data.iter().skip(1) {
            if i & 0b11000000 != 0b10000000 {
                return false;
            }
        }

        true
    }
}
