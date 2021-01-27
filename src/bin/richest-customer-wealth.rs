fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|x| x.iter().sum()).max().unwrap()
    }
}
