#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        enum Direction {
            East,
            South,
            West,
            North,
        }

        let hashset: std::collections::HashSet<_> =
            obstacles.into_iter().map(|x| (x[0], x[1])).collect();

        let (mut p1, mut p2) = (0, 0);
        let mut result = 0;

        let mut direction = Direction::North;

        for i in commands {
            match i {
                -2 => match direction {
                    Direction::East => direction = Direction::North,
                    Direction::South => direction = Direction::East,
                    Direction::West => direction = Direction::South,
                    Direction::North => direction = Direction::West,
                },
                -1 => match direction {
                    Direction::East => direction = Direction::South,
                    Direction::South => direction = Direction::West,
                    Direction::West => direction = Direction::North,
                    Direction::North => direction = Direction::East,
                },
                step => match direction {
                    Direction::East => {
                        for i in 0..step {
                            if hashset.contains(&(p1 + 1, p2)) {
                                break;
                            }
                            p1 += 1;

                            result = result.max(p1 * p1 + p2 * p2);
                        }
                    }
                    Direction::South => {
                        for i in 0..step {
                            if hashset.contains(&(p1, p2 - 1)) {
                                break;
                            }
                            p2 -= 1;

                            result = result.max(p1 * p1 + p2 * p2);
                        }
                    }
                    Direction::West => {
                        for i in 0..step {
                            if hashset.contains(&(p1 - 1, p2)) {
                                break;
                            }
                            p1 -= 1;

                            result = result.max(p1 * p1 + p2 * p2);
                        }
                    }
                    Direction::North => {
                        for i in 0..step {
                            if hashset.contains(&(p1, p2 + 1)) {
                                break;
                            }
                            p2 += 1;

                            result = result.max(p1 * p1 + p2 * p2);
                        }
                    }
                },
            }
        }

        result
    }
}
