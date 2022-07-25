#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    println!("{}", Solution::moving_count(2, 3, 1));
    println!("{}", Solution::moving_count(3, 1, 0));
    println!("{}", Solution::moving_count(3, 2, 17));
    println!("{}", Solution::moving_count(16, 8, 4));
}

struct Solution;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut flags = vec![vec![0; n as usize]; m as usize];
        let mut total = 0;
        for i in 0..m as usize {
            for j in 0..n as usize {
                if Self::sum(i, j) <= k
                    && ((i == 0 && j == 0)
                        || (i > 0 && flags[i - 1][j] == 1)
                        || (j > 0 && flags[i][j - 1] == 1))
                {
                    total += 1;
                    flags[i][j] = 1;
                }
            }
        }

        total
    }

    fn sum(mut x: usize, mut y: usize) -> i32 {
        let mut sum = 0;

        while x >= 10 {
            sum += x % 10;
            x /= 10;
        }

        sum += x;

        while y >= 10 {
            sum += y % 10;
            y /= 10;
        }

        sum += y;

        sum as i32
    }
}

#[test]
fn test_sum() {
    assert_eq!(Solution::sum(14, 15), 11);
    assert_eq!(Solution::sum(101, 230), 7);
}
