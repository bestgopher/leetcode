#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::max_score(vec![2, 3, 6, 1, 9, 2], 5), 13);
    assert_eq!(Solution::max_score(vec![2, 4, 6, 8], 3), 20);
    assert_eq!(
        Solution::max_score(
            vec![
                18, 13, 60, 61, 57, 21, 10, 98, 51, 3, 13, 36, 72, 70, 68, 62, 52, 83, 63, 63, 53,
                42, 59, 98, 95, 48, 22, 64, 94, 80, 14, 14
            ],
            2
        ),
        1633
    );
    assert_eq!(
        Solution::max_score(
            vec![
                38, 92, 23, 30, 25, 96, 6, 71, 78, 77, 33, 23, 71, 48, 87, 77, 53, 28, 6, 20, 90,
                83, 42, 21, 64, 95, 84, 29, 22, 21, 33, 36, 53, 51, 85, 25, 80, 56, 71, 69, 5, 21,
                4, 84, 28, 16, 65, 7
            ],
            52
        ),
        1545
    );
}

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let (mut a, mut b) = (i64::MIN, i64::MIN); // a为奇最大值，b为偶最大值
        if nums[0] % 2 == 0 {
            b = nums[0] as i64;
        } else {
            a = nums[0] as i64;
        }

        let x = x as i64;
        for &i in nums[1..].iter() {
            let i = i as i64;
            if i % 2 == 0 {
                b = (b + i).max(if a == i64::MIN { 0 } else { a } + i - x)
            } else {
                a = (a + i).max(if b == i64::MIN { 0 } else { b } + i - x)
            }
        }

        a.max(b)
    }
}
