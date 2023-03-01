#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
}

struct Solution;

impl Solution {
    /// 先求出前缀和
    /// 然后遍历前缀和列表，如果当前值为k，则说明此位置的前缀和满足条件
    /// 然后再判断是否有当前值-k存在，获取存在的数量
    pub fn subarray_sum(mut nums: Vec<i32>, k: i32) -> i32 {
        for i in 1..nums.len() {
            let (x, y) = (nums[i], nums[i - 1]);
            nums[i] = x + y;
        }

        let mut map = std::collections::HashMap::new();
        let mut ans = 0;
        for i in nums {
            if i == k {
                ans += 1;
            }

            ans += *map.get(&(i - k)).unwrap_or(&0);
            map.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        ans
    }
}
