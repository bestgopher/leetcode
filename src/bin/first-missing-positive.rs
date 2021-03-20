fn main() {}

struct Solution;

impl Solution {
    /// 常规解法，是用hash表
    /// 是用O(n)的空间
    pub fn first_missing_positive1(nums: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::new();

        for i in nums.into_iter() {
            if i > 0 {
                hash.insert(i, ());
            }
        }

        let mut min = 1;

        loop {
            if hash.get(&min).is_some() {
                min += 1;
            } else {
                break;
            }
        }

        min
    }

    /// 当数组中不存在1时，最小的数肯定是1
    /// 当数组中存在非正数或者存在大于n+1的数(n为数组的长度)，则数组中肯定在1~n+1之间有缺失，答案就在1~n+1之间
    /// 如果1~n+1没有数字缺失，答案就是n+1
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        // 首先判断1是否在数组中，如果1不在数组中，则返回1
        if !nums.contains(&1) {
            return 1;
        }

        let mut nums = nums;
        let length = nums.len() as i32 + 1;

        // 将所有<1和大于n+1的数改为1
        for i in nums.iter_mut() {
            if *i < 1 || *i >= length {
                *i = 1
            }
        }

        // 遍历数组，将i对应的下标元素改为负数
        for i in 0..nums.len() {
            let m = nums[i].abs();
            nums[m as usize - 1] = nums[m as usize - 1].abs() * -1;
        }

        let mut result = nums.len() as i32 + 1;

        for (i, v) in nums.into_iter().enumerate() {
            if v > 0 {
                result = i as i32 + 1;
                break;
            }
        }

        result
    }
}
