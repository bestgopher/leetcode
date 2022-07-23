#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!(
        "{}",
        Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2)
    );
    println!(
        "{}",
        Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2, 1], 2)
    );
}

struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0usize;
        let mut index_list: Vec<usize> = vec![]; // 用于存放奇数元素索引的数组
        let k = k as usize;

        for (index, &value) in nums.iter().enumerate() {
            if value % 2 == 1 {
                index_list.push(index);
            }
        }

        if index_list.len() < k {
            return count as i32;
        }

        for (index, &value) in index_list.iter().skip(k - 1).enumerate() {
            let mut s = 0;
            let mut t = 0;
            if index == 0 {
                s = index_list[0] + 1;
            } else {
                s = index_list[index] - index_list[index - 1];
            }

            if index + k == index_list.len() {
                t = nums.len() - value;
            } else {
                t = index_list[index + k] - index_list[index + k - 1];
            }
            count += s * t;
        }

        count as i32
    }
}
