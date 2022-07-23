#![allow(dead_code, unused, unused_variables)]

fn main() {
    let mut v = vec![1, 2, 3];
    Solution::next_permutation(&mut v);
    println!("{:?}", v);

    let mut v = vec![1, 3, 2];
    Solution::next_permutation(&mut v);
    println!("{:?}", v);

    let mut v = vec![2, 3, 1];
    Solution::next_permutation(&mut v);
    println!("{:?}", v);
}

struct Solution;

impl Solution {
    // 遍历数组，找到最后一个处于峰顶的数字
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for i in (0..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                let mut min = i + 1;

                for j in (i + 1..nums.len()).rev() {
                    if nums[j] > nums[i] && nums[min] > nums[j] {
                        min = j;
                    }
                }

                nums.swap(i, min);

                nums[i + 1..].sort();
                return;
            }
        }
        nums.sort();
    }
}
