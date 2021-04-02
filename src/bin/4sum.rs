fn main() {
    // println!("{:?}", Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0));
    // println!("{:?}", Solution::four_sum(vec![0, 0, 0, 0], 0));
    // println!("{:?}", Solution::four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0));
    println!("{:?}", Solution::four_sum(vec![1, -2, -5, -4, -3, 3, 3, 5], -11));
}

struct Solution;

impl Solution {
    /// 第一个数加后面三个数集合
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if nums.len() < 4 {
            return result;
        }

        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() {
            if nums[i] > 0 && nums[i] > target {
                break;
            }

            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let r = Self::three_sum(&nums[i + 1..], target - nums[i]);

            for mut v in r.into_iter() {
                v[0] = nums[i];
                result.push(v);
            }
        }

        result
    }

    fn three_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut r = vec![];
        if nums.len() < 3 {
            return r;
        }

        for i in 0..nums.len() {
            if nums[i] > 0 && nums[i] > target {
                break;
            }

            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, nums.len() - 1);

            while left < right {
                let s = nums[i] + nums[left] + nums[right];
                if s == target {
                    r.push(vec![0, nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if s < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        r
    }
}
