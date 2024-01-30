#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut hash = std::collections::HashMap::<i32, Vec<usize>>::new();
        for (index, value) in nums.into_iter().enumerate() {
            hash.entry(value)
                .and_modify(|x| x.push(index))
                .or_insert(vec![index]);
        }

        let mut result = n;
        for (k, v) in hash {
            let mut mx = v[0] + n - *v.last().unwrap();
            for i in 1..v.len() {
                mx = mx.max(v[i] - v[i - 1]);
            }

            result = result.min(mx / 2);
        }

        result as i32
    }
}
