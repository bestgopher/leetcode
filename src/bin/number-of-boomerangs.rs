#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 3 {
            return 0;
        }

        let mut x: std::collections::HashMap<usize, std::collections::HashMap<i32, i32>> =
            std::collections::HashMap::new();

        for i in 0..points.len() {
            for j in 0..points.len() {
                if i == j {
                    continue;
                }

                let length =
                    (points[i][0] - points[j][0]).pow(2) + (points[i][1] - points[j][1]).pow(2);

                x.entry(i)
                    .and_modify(|m| {
                        m.entry(length).and_modify(|x| *x += 1).or_insert(1);
                    })
                    .or_insert({
                        let mut m = std::collections::HashMap::new();
                        m.insert(length, 1);
                        m
                    });
            }
        }

        let mut result = 0;
        for v in x.values() {
            for x in v.values() {
                result += (1 + x - 1) * (x - 1);
            }
        }

        result
    }
}
