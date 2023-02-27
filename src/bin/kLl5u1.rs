#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 简单方式：记录每个数的下标，
    /// 如果当前数为current，然后通过查找target-current的下标，如果存在则说明就是这两个数
    pub fn two_sum1(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, v) in numbers.into_iter().enumerate() {
            if let Some(&x) = map.get(&(target - v)) {
                return vec![x, i as i32];
            }

            map.insert(v, i as i32);
        }

        unreachable!()
    }

    /// 双指针。
    /// 因为数是已排序的，则两个指针，一个从前往后，一个从后往前遍历即可
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut p1, mut p2) = (0, numbers.len() - 1);
        while p1 < p2 {
            match (numbers[p1] + numbers[p2]).cmp(&target) {
                std::cmp::Ordering::Equal => return vec![p1 as i32, p2 as i32],
                std::cmp::Ordering::Less => p1 += 1,
                std::cmp::Ordering::Greater => p2 -= 1,
            }
        }

        unreachable!()
    }
}
