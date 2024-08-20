#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 1. 先去重
    /// 2. 移除都存在的数
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let l = nums1.len() as i32 / 2;
        let n1 = nums1.into_iter().collect::<std::collections::HashSet<_>>();
        let n2 = nums2.into_iter().collect::<std::collections::HashSet<_>>();

        let mut comm = n1.intersection(&n2).count() as i32;
        let mut result = n1.len() as i32 + n2.len() as i32 - comm; // 所有元素的数量

        if n1.len() as i32 > l {
            let nm = comm.min(n1.len() as i32 - l); // 如果交集大于comm，则全部移除交集，否则，移除多余的n1的元素
            result -= n1.len() as i32 - nm - l;
            comm -= nm;
        }

        if n2.len() as i32 > l {
            result -= (n2.len() as i32 - comm.min(n2.len() as i32 - l)) - l;
        }

        result
    }
}
