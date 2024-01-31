#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut count = std::collections::HashMap::new();
        for &i in nums.iter() {
            count.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut result = Vec::with_capacity(nums.len());
        let mut set = std::collections::HashSet::new();

        for i in nums {
            set.insert(i);
            let c = count[&i];
            if c == 1 {
                count.remove(&i);
            } else {
                count.insert(i, c - 1);
            }

            result.push((set.len() - count.len()) as i32);
        }

        result
    }
}
