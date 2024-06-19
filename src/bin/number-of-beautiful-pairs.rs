#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::count_beautiful_pairs(vec![84, 91, 18, 59, 27, 9, 81, 33, 17, 58],),
        37
    );
}

struct Solution;

impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let m = [
            [0; 10],
            [0, 1, 1, 1, 1, 1, 1, 1, 1, 1], // 1
            [0, 1, 0, 1, 0, 1, 0, 1, 0, 1], // 2
            [0, 1, 1, 0, 1, 1, 0, 1, 1, 0], // 3
            [0, 1, 0, 1, 0, 1, 0, 1, 0, 1], // 4
            [0, 1, 1, 1, 1, 0, 1, 1, 1, 1], // 5
            [0, 1, 0, 0, 0, 1, 0, 1, 0, 0], // 6
            [0, 1, 1, 1, 1, 1, 1, 0, 1, 1], // 7
            [0, 1, 0, 1, 0, 1, 0, 1, 0, 1], // 8
            [0, 1, 1, 0, 1, 1, 0, 1, 1, 0], // 9
        ];

        let f = |mut x: i32| -> i32 {
            while x > 9 {
                x /= 10;
            }
            x
        };

        let mut result = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let x = (nums[j] % 10) as usize;
                let y = f(nums[i]) as usize;

                if m[x][y] == 1 {
                    result += 1;
                }
            }
        }

        result
    }
}
