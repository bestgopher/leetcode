#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{}", !10);
}

struct Solution;

impl Solution {
    /// 穷举，暴力破解
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                if !((arr[i] - arr[j]).abs() <= a) {
                    continue;
                }

                for k in j + 1..arr.len() {
                    if (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}
