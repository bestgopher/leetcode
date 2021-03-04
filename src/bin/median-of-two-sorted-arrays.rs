fn main() {}

struct Solution;

/// 如果两个数组的长度之和为奇数时，则中位数为合并后数组中间那个数
/// 如果两个数组的长度之和为偶数时，则中位数为中间两个数之和的平均数
impl Solution {
    /// 方法1：按升序合并两个数组，取中位数
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() == 1 && nums2.is_empty() {
            return nums1[0] as f64;
        }

        if nums2.len() == 1 && nums1.is_empty() {
            return nums2[0] as f64;
        }

        let mut v = Vec::with_capacity(nums1.len() + nums2.len());
        let (mut index1, mut index2) = (0, 0);

        while index1 < nums1.len() || index2 < nums2.len() {
            if index1 < nums1.len() && index2 < nums2.len() {
                if nums1[index1] < nums2[index2] {
                    v.push(nums1[index1]);
                    index1 += 1;
                } else {
                    v.push(nums2[index2]);
                    index2 += 1;
                }
            } else if index1 < nums1.len() {
                v.push(nums1[index1]);
                index1 += 1;
            } else {
                v.push(nums2[index2]);
                index2 += 1;
            }
        }

        if v.len() % 2 == 1 {
            v[v.len() / 2] as f64
        } else {
            (v[v.len() / 2 - 1] as f64 + v[v.len() / 2] as f64) / 2f64
        }
    }
}
