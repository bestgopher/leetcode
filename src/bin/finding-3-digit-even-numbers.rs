#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 枚举100-999的数据
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut hash = std::collections::HashMap::new();

        for i in digits {
            hash.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut result = vec![];
        for j in 100..=999 {
            if j % 2 == 1 {
                continue;
            }

            let x = j / 100;
            let y = j / 10 % 10;
            let z = j % 10;

            if x != y && y != z && x != z {
                if hash.contains_key(&x) && hash.contains_key(&y) && hash.contains_key(&z) {
                    result.push(j);
                }
            } else if x == y && y != z {
                match (hash.get(&x), hash.get(&z)) {
                    (Some(m), Some(_)) if *m >= 2 => {
                        result.push(j);
                    }
                    _ => {}
                }
            } else if x == z && z != y {
                match (hash.get(&x), hash.get(&y)) {
                    (Some(m), Some(_)) if *m >= 2 => {
                        result.push(j);
                    }
                    _ => {}
                }
            } else if y == z && z != x {
                match (hash.get(&z), hash.get(&x)) {
                    (Some(m), Some(_)) if *m >= 2 => {
                        result.push(j);
                    }
                    _ => {}
                }
            } else {
                match hash.get(&x) {
                    Some(m) if *m >= 3 => {
                        result.push(j);
                    }
                    _ => {}
                }
            }
        }

        result
    }
}
