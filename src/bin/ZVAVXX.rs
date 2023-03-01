#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
        8
    );
    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
        0
    );

    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![1, 1, 1], 2),
        6
    );
}

struct Solution;

impl Solution {
    /// 两个指针 start和end，start和end的子数组的积为 product
    /// 先以start为起点，递增end，判断 product 是否满足条件，不满足条件的时候停止递增
    /// 然后递增start，直到start等于end为止，
    /// 然后重复2，3步直到最后一步
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut start = 0;
        let mut product = 1;
        let mut ans = 0;
        for (i, &v) in nums.iter().enumerate() {
            product *= v;

            while start <= i && product >= k {
                product /= nums[start];
                start += 1;
            }

            ans += (i as i32 - start as i32) + 1;
        }

        ans
    }
}
