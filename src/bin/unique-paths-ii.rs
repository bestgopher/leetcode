#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut r = vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];
        Self::scan(&mut r, &obstacle_grid, 0, 0);
        if r[0][0] != -1 {
            r[0][0]
        } else {
            0
        }
    }

    fn scan(v: &mut Vec<Vec<i32>>, ob: &Vec<Vec<i32>>, index1: usize, index2: usize) {
        let (s1, s2) = (v.len() - 1, v[0].len() - 1);
        if index1 == s1 && index2 == s2 && ob[index1][index2] != 1 {
            v[index1][index2] = 1;
            return;
        }

        if index1 > s1 || index2 > s2 || v[index1][index2] != 0 {
            return;
        }

        // 当存在障碍物时
        if ob[index1][index2] == 1 {
            v[index1][index2] = -1;
            return;
        }

        Self::scan(v, ob, index1 + 1, index2);
        Self::scan(v, ob, index1, index2 + 1);

        if index1 < s1 {
            v[index1][index2] += if v[index1 + 1][index2] != -1 {
                v[index1 + 1][index2]
            } else {
                0
            };
        }

        if index2 < s2 {
            v[index1][index2] += if v[index1][index2 + 1] != -1 {
                v[index1][index2 + 1]
            } else {
                0
            };
        }
    }
}
