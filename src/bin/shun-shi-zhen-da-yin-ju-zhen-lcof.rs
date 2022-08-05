#![allow(dead_code, unused, unused_variables, non_snake_case)]

use serde::__private::de;

fn main() {}

struct Solution;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }

        let (mut left, mut right, mut up, mut down) = (0, matrix[0].len(), 0, matrix.len());
        let mut data = Vec::with_capacity(right * down);
        let (mut index1, mut index2) = (0, 0);

        let mut direction = Direction::Right;

        while left < right && up < down {
            data.push(matrix[index1][index2]);
            match direction {
                Direction::Right => {
                    if index2 < right - 1 {
                        index2 += 1;
                    } else {
                        direction = Direction::Down;
                        index1 += 1;
                        up += 1;
                    }
                }

                Direction::Down => {
                    if index1 < down - 1 {
                        index1 += 1;
                    } else {
                        direction = Direction::Left;
                        index2 -= 1;
                        right -= 1;
                    }
                }
                Direction::Left => {
                    if index2 > left + 1 {
                        index2 -= 1;
                    } else {
                        direction = Direction::Up;
                        index1 -= 1;
                        down -= 1;
                    }
                }
                Direction::Up => {
                    if index1 > up + 1 {
                        index1 -= 1;
                    } else {
                        direction = Direction::Right;
                        index2 += 1;
                        left += 1;
                    }
                }
            }
        }

        data
    }
}
