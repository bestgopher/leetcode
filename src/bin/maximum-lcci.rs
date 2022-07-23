#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(10, Solution::maximum(5, 10));
}

struct Solution;

impl Solution {
    pub fn maximum(a: i32, b: i32) -> i32 {
        // 当a > b时，最高位为0
        // 当a < b时，a - b为负数，最高位为1
        // 因此求出最高为是0还是1
        let ret = ((a as i64 - b as i64) as u64 >> 63) as i32;
        // 当最高位为1时，说明b>a，因此ret^1 == 0,所以下面表达式就为b * 1
        // 当最高位为0时，说明b<a，因此ret^1 = 0 ^ 1 => 1,所以下边的表达式为1 * a
        b * ret + (ret ^ 1) * a
    }
}
