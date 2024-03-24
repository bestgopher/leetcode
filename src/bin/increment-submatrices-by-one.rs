#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 二维差分
    /// 开始的下标+1，结尾+1的下标-1
    /// 例如有数组 vec![0,0,0,0,0,0,0,0],如果把[1,3]的下标都加一，因此我们有差分数组vec![0,1,0,0,-1,0,0,0]
    /// 然后每个下标为前缀和
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut diff = vec![vec![0; n as usize + 2]; n as usize + 2];

        for query in queries {
            let (r1, c1, r2, c2) = (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3] as usize,
            );

            diff[r1 + 1][c1 + 1] += 1;
            diff[r1 + 1][c2 + 2] -= 1;
            diff[r2 + 2][c1 + 1] -= 1;
            diff[r2 + 2][c2 + 2] += 1;
        }

        for i in 1..=n as usize {
            for j in 1..=n as usize {
                diff[i][j] += diff[i][j - 1] + diff[i - 1][j] - diff[i - 1][j - 1];
            }
        }

        diff[1..n as usize + 1]
            .into_iter()
            .map(|x| x[1..n as usize + 1].to_vec())
            .collect()
    }
}
