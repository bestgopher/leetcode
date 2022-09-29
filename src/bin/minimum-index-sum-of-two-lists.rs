#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let m = list1
            .into_iter()
            .enumerate()
            .map(|x| (x.1, x.0))
            .collect::<std::collections::HashMap<String, usize>>();

        let mut data = vec![];
        let mut index = usize::MAX;

        for (i, v) in list2.into_iter().enumerate() {
            if i > index {
                break;
            }

            if let Some(&x) = m.get(&v) {
                if i + x <= index {
                    if i + x < index {
                        data.clear();
                    }
                    data.push(v);
                    index = i + x;
                }
            }
        }

        data
    }
}
