fn main() {}

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut hash = std::collections::HashMap::new();
        for (i, v) in nums.into_iter().enumerate() {
            if let Some(i1) = hash.get(&v) {
                if i - i1 <= k as usize {
                    return true;
                }
            }

            hash.insert(v, i);
        }

        false
    }
}
