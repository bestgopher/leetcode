fn main() {
    assert_eq!(28, Solution::unique_paths(3, 7));
}

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut v = vec![vec![0; m as usize]; n as usize];

        Self::calc(&mut v, m, n);

        v[0][0]
    }

    fn calc(array: &mut Vec<Vec<i32>>, m: i32, n: i32) {



    }
}
