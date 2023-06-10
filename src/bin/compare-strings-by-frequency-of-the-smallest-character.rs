#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let s1 = queries
            .into_iter()
            .map(|x| Self::count(x.as_bytes()))
            .collect::<Vec<_>>();

        let s2 = words
            .into_iter()
            .map(|x| Self::count(x.as_bytes()))
            .collect::<Vec<_>>();

        let mut result = vec![];

        for i in s1 {
            let mut count = 0;

            for j in s2.iter() {
                if i < *j {
                    count += 1;
                }
            }

            result.push(count);
        }

        result
    }

    fn count(s: &[u8]) -> i32 {
        let mut count = 1;
        let mut min = s[0];

        for &i in &s[1..] {
            if i < min {
                min = i;
                count = 1;
            } else if i == min {
                count += 1;
            }
        }

        count
    }
}
