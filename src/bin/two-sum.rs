fn main() {}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = std::collections::HashMap::new();
        let mut result = Vec::with_capacity(2);
        for (index, value) in nums.into_iter().enumerate() {
            if let Some(&i) = hash.get(&(target - value)) {
                result.push(i as i32);
                result.push(index as i32);
            } else {
                hash.insert(value, index);
            }
        }
        result
    }
}
