#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 使用heap保存前面出现的负数，然后记录所有的血量之和，当血量为负数时，说明到此位置的生时候需要调整。
    /// 我们弹出heap中血量最小的(也就是减去血量最多的)，然后放到末尾，这时血量之和应该把此血量减去的加上，如果为正数，停止调整。
    pub fn magic_tower(nums: Vec<i32>) -> i32 {
        use std::cmp::Reverse;

        let mut heap = std::collections::BinaryHeap::new();
        let mut total = 1i64; // 初始血量为1
        let mut num = 0;
        let mut total_sum = 0i64;
        for i in nums {
            total_sum += i as i64;
            if i < 0 {
                heap.push(Reverse(i));
            }

            total += i as i64;

            if total <= 0 {
                while let Some(Reverse(x)) = heap.pop() {
                    num += 1;
                    total += -x as i64;
                    if total > 0 {
                        break;
                    }
                }
            }
        }

        if total_sum >= 0 {
            num
        } else {
            -1
        }
    }
}
