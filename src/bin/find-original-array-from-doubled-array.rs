#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let mut changed = changed;
        changed.sort();
        let mut map = std::collections::HashMap::new();
        let mut result = vec![];

        for i in changed {
            if let Some(x) = map.get_mut(&i) {
                if *x > 0 {
                    *x -= 1;
                }

                if *x == 0 {
                    map.remove(&i);
                }
            } else {
                result.push(i);
                map.entry(i * 2).and_modify(|x| *x += 1).or_insert(1);
            }
        }

        if map.is_empty() {
            result
        } else {
            vec![]
        }
    }
}
