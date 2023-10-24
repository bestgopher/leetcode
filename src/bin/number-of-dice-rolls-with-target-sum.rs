#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        if target < n || target > n * k {
            return 0; // 无法组成 target
        }
        const MOD: i32 = 1_000_000_007;
        let (n, k, target) = (n as usize, k as usize, target as usize);
        let mut f = vec![vec![0; target - n + 1]; n + 1];
        f[0][0] = 1;
        for i in 1..=n {
            for j in 0..=target - n {
                for x in 0..k.min(j + 1) {
                    // 掷出了 x
                    f[i][j] = (f[i][j] + f[i - 1][j - x]) % MOD;
                }
            }
        }
        f[n][target - n]
    }
}
