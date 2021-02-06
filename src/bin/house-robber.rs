fn main() {}

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut scores = vec![-1; nums.len()];
        Self::dp(&nums, &mut scores);
        scores[0]
    }

    fn dp(nums: &Vec<i32>, scores: &mut Vec<i32>) {
        let s = scores.len();
        if nums.len() == 1 {
            scores[s - 1] = nums[0];
            return;
        }

        if nums.len() == 2 {
            scores[s - 2] = std::cmp::max(nums[0], nums[1]);
            return;
        }

        let mut n1 = 0;
        if scores[s - nums.len() + 2] != -1 {
            n1 = nums[0] + scores[s - nums.len() + 2];
        } else {
            Self::dp(&nums[2..].to_vec(), scores);
            n1 = nums[0] + scores[s - nums.len() + 2];
        }

        let mut n2 = 0;
        if scores[s - nums.len() + 1] != -1 {
            n2 = scores[s - nums.len() + 1];
        } else {
            Self::dp(&nums[1..].to_vec(), scores);
            n2 = scores[s - nums.len() + 1];
        }
        scores[s-nums.len()] = std::cmp::max(n1, n2);
    }
}
