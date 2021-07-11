fn main() {}

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..nums.len() {
            let sum = nums[i];
            let mut hash = std::collections::HashMap::new();
            max = max.max(sum + Self::dp(&nums[..], i, i + 2, &mut hash));
            max = max.max(sum + Self::dp(&nums[..], i, i + 3, &mut hash));
        }

        max
    }

    fn dp(
        nums: &[i32],
        start: usize,
        now_index: usize,
        hash: &mut std::collections::HashMap<usize, i32>,
    ) -> i32 {
        let now_index = if now_index >= nums.len() {
            now_index % nums.len()
        } else {
            now_index
        };
        if (start == 0 && now_index == nums.len() - 1)
            || now_index == start
            || now_index == start + 1
            || (start != 0 && now_index == start - 1)
            || (start == nums.len() - 1 && now_index == 0)
        {
            return 0;
        }

        let index1 = now_index + 2;
        let v1 = if let Some(&x) = hash.get(&index1) {
            x
        } else {
            Self::dp(nums, start, index1, hash)
        };

        let index2 = now_index + 3;
        let v2 = if let Some(&x) = hash.get(&index2) {
            x
        } else {
            Self::dp(nums, start, index2, hash)
        };

        hash.insert(now_index, nums[now_index] + v1.max(v2));

        nums[now_index] + v1.max(v2)
    }
}
