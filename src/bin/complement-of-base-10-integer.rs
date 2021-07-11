fn main() {}

struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut ones = 1;
        while ones < n {
            ones = (ones << 1) + 1
        }
        ones ^ n
    }
}
