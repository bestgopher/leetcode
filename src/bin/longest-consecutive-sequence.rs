#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    // assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(
        Solution::hashset_longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
        9
    );
}

struct Solution;

impl Solution {
    /// 使用哈希集合
    /// 遍历到一个数的时候，挨个+1看集合中是否存在，存在则说明是连续的。
    /// 优化：如果集合中是 [2，3，1，4，5] 的话，我们遍历2的时候，检查3，4，5是否存在，但是遍历到1的时候，要检查2，3，4，5是否存在，因此我们不需要在2的时候进行检查。因此遍历到i时，如果i-1存在，则i的时候不需要检查。
    pub fn hashset_longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = nums
            .iter()
            .map(|x| *x)
            .collect::<std::collections::HashSet<i32>>();

        let mut ans = 0;

        for mut i in nums {
            if set.contains(&(i - 1)) {
                continue;
            }

            let mut r = 0;
            while set.contains(&i) {
                r += 1;
                i += 1;
            }

            ans = ans.max(r);
        }

        ans
    }

    /// 使用hash表，保存值对应的最远有边界，比如 [2，3，1，4，5], 2的最远右边界就是5，1的最远右边界也是5.
    /// 初始化的时候，每个数的最远右边界为自己。
    /// 遍历的时候，如果遍历到i，如果i-1存在，则说明i是一个连续序列的起点，获取到对应的右边界 right1
    /// 如果right1+1存在， 获取到right1+1的右边界的值right2，如果 right2 + 1 存在，则获取right2+1的右边界right3，如果right3+1存在...
    /// 此时i的右边界则为最后一个值
    pub fn hashmap_longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut map = nums
            .iter()
            .map(|x| (*x, *x))
            .collect::<std::collections::HashMap<i32, i32>>();

        let mut ans = 0;

        for i in nums {
            if map.contains_key(&(i - 1)) {
                continue;
            }

            let mut right = *map.get(&i).unwrap();

            while map.contains_key(&(right + 1)) {
                right = *map.get(&(right + 1)).unwrap();
            }

            ans = ans.max(right - i + 1);
            map.insert(i, right);
        }

        ans
    }

    /// hash表存数字i所在的连续最大长度值
    /// 对于数字i不存在，则i的值为 i-1 与 i+1的最大值 +1
    /// 然后更新i-1、i+1、i
    fn dp_longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::<i32, i32>::new();
        let mut ans = 0;

        for i in nums {
            if map.contains_key(&i) {
                continue;
            }

            let left = map.get(&(i - 1)).and_then(|x| Some(*x)).unwrap_or_default();
            let right = map.get(&(i + 1)).and_then(|x| Some(*x)).unwrap_or_default();

            let current = 1 + left + right;
            ans = ans.max(current);
            map.insert(i, current);
            map.get_mut(&(i + right)).map(|x| *x = current);
            map.get_mut(&(i - left)).map(|x| *x = current);
        }

        ans
    }
}
