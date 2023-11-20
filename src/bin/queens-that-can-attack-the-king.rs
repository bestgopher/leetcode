#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::collections::HashSet;

use serde::de::Unexpected::Option;
use tera::Filter;

fn main() {}

struct Solution;

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let hash: HashSet<_> = queens.into_iter().collect();

        // 枚举8个方向，遇到的第一个就行
        let mut result = vec![];
        let mut s = king[0];
        while s >= 1 {
            if hash.contains(&vec![s - 1, king[1]]) {
                result.push(vec![s - 1, king[1]]);
                break;
            }
            s -= 1;
        }

        let mut s = king[0];
        while s <= 6 {
            if hash.contains(&vec![s + 1, king[1]]) {
                result.push(vec![s + 1, king[1]]);
                break;
            }
            s += 1;
        }

        let mut s = king[1];
        while s >= 1 {
            if hash.contains(&vec![king[0], s - 1]) {
                result.push(vec![king[0], s - 1]);
                break;
            }
            s -= 1;
        }

        let mut s = king[1];
        while s <= 6 {
            if hash.contains(&vec![king[0], s + 1]) {
                result.push(vec![king[0], s + 1]);
                break;
            }
            s += 1;
        }

        let (mut s1, mut s2) = (king[0], king[1]);
        while s1 >= 1 && s2 >= 1 {
            if hash.contains(&vec![s1 - 1, s2 - 1]) {
                result.push(vec![s1 - 1, s2 - 1]);
                break;
            }
            s1 -= 1;
            s2 -= 1;
        }

        let (mut s1, mut s2) = (king[0], king[1]);
        while s1 <= 6 && s2 <= 6 {
            if hash.contains(&vec![s1 + 1, s2 + 1]) {
                result.push(vec![s1 + 1, s2 + 1]);
                break;
            }
            s1 += 1;
            s2 += 1;
        }

        let (mut s1, mut s2) = (king[0], king[1]);
        while s1 >= 0 && s2 <= 6 {
            if hash.contains(&vec![s1 - 1, s2 + 1]) {
                result.push(vec![s1 - 1, s2 + 1]);
                break;
            }
            s1 -= 1;
            s2 += 1;
        }

        let (mut s1, mut s2) = (king[0], king[1]);
        while s1 <= 6 && s2 >= 0 {
            if hash.contains(&vec![s1 + 1, s2 - 1]) {
                result.push(vec![s1 + 1, s2 - 1]);
                break;
            }
            s1 += 1;
            s2 -= 1;
        }

        result
    }
}
