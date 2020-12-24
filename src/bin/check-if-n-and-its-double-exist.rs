fn main() {}

struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::<i32, usize>::new();

        for (index, value) in arr.iter().enumerate() {
            map.insert(*value, index);
        }

        for (index, value) in arr.iter().map(|x| x * 2).enumerate() {
            if let Some(i) = map.get(&value) {
                if *i != index {
                    return true;
                }
            }
        }

        false
    }
}