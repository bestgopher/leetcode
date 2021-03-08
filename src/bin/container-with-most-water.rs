fn main() {
    assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    assert_eq!(16, Solution::max_area(vec![4, 3, 2, 1, 4]));
    assert_eq!(2, Solution::max_area(vec![1, 2, 1]));
}

struct Solution;

impl Solution {
    /// 双指针，第一个指针指向开始的元素，第二个指针指向最后一个元素，则此时的面积为:
    ///     第一个元素和最后一个元素的最小值 * 下标之差
    /// 然后比较两个元素，第一个元素小，则第一个指针+1
    /// 第二个元素小，则第二个指针-1
    /// 然后再计算此时的面积，取与存在的面积的最大值
    /// 依次计算，直到两个指针相等为止
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut pointer1, mut pointer2) = (0usize, height.len() - 1);
        let mut area = (pointer2 - pointer1) as i32 * height[pointer2].min(height[pointer1]);
        while pointer1 != pointer2 {
            if height[pointer1] <= height[pointer2] {
                pointer1 += 1;
            } else {
                pointer2 -= 1;
            }

            area = area.max((pointer2 - pointer1) as i32 * height[pointer2].min(height[pointer1]));
        }

        area
    }
}
