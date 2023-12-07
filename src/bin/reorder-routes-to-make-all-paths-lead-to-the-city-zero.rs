#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

#[derive(Clone)]
enum Direction {
    Go(i32),
    Back(i32),
}

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut c = vec![Vec::new(); n as usize];

        for i in connections {
            c[i[0] as usize].push(Direction::Go(i[1]));
            c[i[1] as usize].push(Direction::Back(i[0]));
        }

        Self::dfs(0, -1, &c[..])
    }

    fn dfs(x: usize, parent: i32, connections: &[Vec<Direction>]) -> i32 {
        let mut res = 0;
        for i in connections[x].iter() {
            match *i {
                Direction::Back(d) if d != parent => {
                    res += Self::dfs(d as usize, x as i32, connections);
                }
                Direction::Go(d) if d != parent => {
                    res += Self::dfs(d as usize, x as i32, connections) + 1;
                }
                _ => {}
            }
        }

        res
    }
}
