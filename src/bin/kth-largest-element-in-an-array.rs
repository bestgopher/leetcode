fn main() {
    Solution::find_kth_largest(vec![1], 1);
}

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        Self::heapify(&mut nums);

        for i in 1..k as usize {
            Self::heapify(&mut nums[i..])
        }

        nums[k as usize - 1]
    }

    fn heapify(nums: &mut [i32]) {
        let mut index = (nums.len() - 1) / 2;

        for i in (0..=index).rev() {
            Self::down_heap(nums, i);
        }
    }

    fn down_heap(nums: &mut [i32], mut index: usize) {
        while index * 2 + 1 < nums.len() {
            let mut max = index * 2 + 1;
            if index * 2 + 2 < nums.len() && nums[index * 2 + 1] < nums[index * 2 + 2] {
                max = index * 2 + 2;
            }

            if nums[max] > nums[index] {
                nums.swap(max, index);
                index = max;
            } else {
                break;
            }
        }
    }
}
