#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
    assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    assert_eq!(
        Solution::asteroid_collision(vec![-2, -1, 1, 2]),
        vec![-2, -1, 1, 2]
    );
}

struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut v = vec![];

        for i in asteroids {
            v.push(i);
            while v.len() > 1 {
                let l1 = v[v.len() - 1];
                if l1 > 0 {
                    break;
                }

                let l2 = v[v.len() - 2];

                if l1 < 0 && l2 > 0 {
                    if l1.abs() > l2 {
                        let i = v.len() - 2;
                        v[i] = l1;
                        v.pop();
                    } else if l1.abs() == l2 {
                        v.pop();
                        v.pop();
                        break;
                    } else {
                        v.pop();
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        v
    }
}
