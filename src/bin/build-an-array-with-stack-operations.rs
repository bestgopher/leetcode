fn main() {}

struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut index = 0;
        let mut result = Vec::<String>::with_capacity(target.len());
        for i in 1..=n {
            result.push(String::from("Push"));
            if target[index] == i {
                if index == target.len() - 1 {
                    break;
                }
                index += 1;
            } else {
                result.push(String::from("Pop"));
            }
        }

        result
    }
}