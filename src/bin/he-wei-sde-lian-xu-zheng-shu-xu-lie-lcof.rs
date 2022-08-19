#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 从i=1，j=2开始，代表连续数组的边界值，sum为3，此时sum是最小的
    /// 如果sum大于target，则说明i需要增加，i增加则sum减少i
    /// 如果sum小于target，则说明j需要增加，j增加则sum增加j
    /// 当i>=j时，终止
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let (mut i, mut j) = (1, 2);
        let mut sum = i + j;
        let mut v = vec![];
        while i < j {
            if sum == target {
                v.push((i..=j).collect());
                sum -= i;
                i += 1;
                j += 1;
                sum += j;
            } else if sum > target {
                sum -= i;
                i += 1;
            } else {
                j += 1;
                sum += j;
            }
        }

        v
    }
}
