fn main() {}

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut s = std::collections::HashMap::new();

        for &i in nums.iter() {
            s.insert(i, ());
        }

        let mut v = vec![];
        for i in 1..=nums.len() as i32 {
            if s.get(&i).is_none() {
                v.push(i);
            }
        }

        v
    }
}
