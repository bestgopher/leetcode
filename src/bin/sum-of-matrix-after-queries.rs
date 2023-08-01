#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 正难则反，倒序操作
    pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
        use std::collections::HashMap;
        // r1行 r2列
        let (mut r1, mut r2) = (HashMap::new(), HashMap::new());
        let mut sum = 0;
        for i in queries.into_iter().rev() {
            match i[0] {
                0 => {
                    if r1.contains_key(&i[1]) {
                        continue;
                    }

                    sum += (n as i64 - r2.len() as i64) * i[2] as i64;
                    r1.insert(i[1], ());
                }
                1 => {
                    if r2.contains_key(&i[1]) {
                        continue;
                    }

                    sum += (n as i64 - r1.len() as i64) * i[2] as i64;
                    r2.insert(i[1], ());
                }
                _ => unreachable!(),
            }
        }

        sum
    }
}
