fn main() {}

struct Solution;

impl Solution {
    pub fn lexical_order1(n: i32) -> Vec<i32> {
        let mut s = (1..=n)
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        s.sort();

        s.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }

    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut v = Vec::with_capacity(n as usize);

        for i in 1..10.min(n + 1) {
            v.push(i);
            Self::dfs(&mut v, i, n);
        }
        v
    }

    fn dfs(v: &mut Vec<i32>, number: i32, n: i32) {
        if number > n { return; }

        let number = number * 10;

        for i in 0..10 {
            if number + i <= n {
                v.push(number + i);
                Self::dfs(v, number + i, n);
            }
        }
    }
}
