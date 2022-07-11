fn main() {}

struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let j = jewels
            .bytes()
            .into_iter()
            .collect::<std::collections::HashSet<u8>>();
        stones.bytes().into_iter().filter(|x| j.contains(x)).count() as i32
    }
}
