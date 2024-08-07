#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

/*
 * // This is the custom function interface.
 * // You should not implement it, or speculate about its implementation
 */
struct CustomFunction;
impl CustomFunction {
    pub fn f(x: i32, y: i32) -> i32 {
        unimplemented!()
    }
}
impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let (mut x, mut y) = (1, 1000);

        while x < 1000 && y > 0 {
            let r = CustomFunction::f(x, y);
            if r > z {
                y -= 1;
            } else if r < z {
                x += 1;
            } else {
                result.push(vec![x, y]);
                x += 1;
                y -= 1;
            }
        }

        result
    }
}
