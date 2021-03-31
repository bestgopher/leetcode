fn main() {
    // println!("{:?}", Solution::three_sum(vec![0, 0, 0, 0]));
    // println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    // println!("{:?}", Solution::three_sum(vec![1, -1, -1, 0]));
    println!("{:?}", Solution::three_sum(vec![-2, 0, 0, 2, 2]));
}

struct Solution;

impl Solution {
    /// 先排序
    /// 然后在从头开始找
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        if nums.len() < 3 {
            return result;
        }

        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                break;
            }

            // 当前后两个数相同时，第一个数已经把这个数的情况给计算在内了，所以直接跳转到下一个数字
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, nums.len() - 1);

            while left < right {
                match 0.cmp(&(nums[i] + nums[left] + nums[right])) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;

                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }

                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    }
                    std::cmp::Ordering::Less => right -= 1,
                    std::cmp::Ordering::Greater => left += 1,
                }
            }
        }

        result
    }
}
