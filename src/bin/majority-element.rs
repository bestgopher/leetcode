fn main() {}

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();

        for i in nums.iter() {
            if let Some(x) = m.get_mut(i) {
                if *x + 1 > nums.len() / 2 {
                    return *i;
                } else {
                    *x += 1;
                }
            } else {
                if 1 > nums.len() / 2 {
                    return *i;
                }
                m.insert(i, 1usize);
            }
        }

        -1
    }
}
