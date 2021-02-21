fn main() {}

struct Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        // v[0]为从数组竖直方向（即顶部，底部）看
        // v[1]为从数组水平方向（即顶部，底部）看
        let mut v = vec![vec![0; grid[0].len()]; 2];
        // sum1 增加后的总和
        // sum2 增加前的总和
        let (mut sum1, mut sum2) = (0, 0);

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > v[0][j] {
                    v[0][j] = grid[i][j];
                }

                if grid[i][j] > v[1][i] {
                    v[1][i] = grid[i][j];
                }
                sum2 += grid[i][j];
            }
        }

        // 新的二维数组的节点值为天际线数组对应值的最小值
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                sum1 += v[0][j].min(v[1][i]);
            }
        }

        sum1 - sum2
    }
}
