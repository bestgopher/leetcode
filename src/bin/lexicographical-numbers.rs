fn main() {}

struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut s = (1..=n)
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        s.sort();

        s.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }
}
