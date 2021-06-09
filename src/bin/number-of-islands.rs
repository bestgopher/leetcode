fn main() {}

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut r = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    r += 1;
                    Self::change_1_to_0(&mut grid, (i, j));
                }
            }
        }

        r
    }

    fn change_1_to_0(grid: &mut Vec<Vec<char>>, index: (usize, usize)) {
        if index.0 == grid.len() || index.1 == grid[0].len() {
            return;
        }

        if grid[index.0][index.1] == '0' {
            return;
        }

        grid[index.0][index.1] = '0';

        Self::change_1_to_0(grid, (index.0 + 1, index.1));
        Self::change_1_to_0(grid, (index.0, index.1 + 1));

        if index.0 > 0 {
            Self::change_1_to_0(grid, (index.0 - 1, index.1));
        }

        if index.1 > 0 {
            Self::change_1_to_0(grid, (index.0, index.1 - 1));
        }
    }
}
