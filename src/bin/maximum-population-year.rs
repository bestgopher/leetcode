#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut v = vec![0; 1000];

        for i in logs {
            for x in (i[0] - 1950) as usize..(i[1] - 1950) as usize {
                v[x] += 1;
            }
        }

        let mut max = 0;

        for i in 1..v.len() {
            max = if v[i] > v[max] { i } else { max }
        }

        max as i32 + 1950
    }
}
