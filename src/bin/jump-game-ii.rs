#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{}", Solution::jump(vec![2, 3, 1, 1, 4]));
    println!("{}", Solution::jump(vec![1, 1, 1, 1]));
    println!(
        "{}",
        Solution::jump(vec![
            5, 6, 4, 4, 6, 9, 4, 4, 7, 4, 4, 8, 2, 6, 8, 1, 5, 9, 6, 5, 2, 7, 9, 7, 9, 6, 9, 4, 1,
            6, 8, 8, 4, 4, 2, 0, 3, 8, 5
        ])
    );
    println!(
        "{}",
        Solution::jump(vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0])
    );
}

struct Solution;

impl Solution {
    /// 常规解法，用hash表记录每个下标到终点的最小距离
    pub fn jump1(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }

        // 保存下标到终点的最小值
        let mut hash = std::collections::HashMap::<usize, i32>::with_capacity(nums.len());
        Self::step(nums.as_ref(), 0, &mut hash);
        *hash.get(&0).unwrap()
    }

    fn step(nums: &Vec<i32>, start: usize, hash: &mut std::collections::HashMap<usize, i32>) {
        if hash.get(&start).is_some() {
            return;
        }

        for i in 1..=nums[start] {
            if start + i as usize == nums.len() - 1 {
                hash.insert(start, 1);
                return;
            }

            if nums[start + i as usize] == 0 {
                continue;
            }

            Self::step(nums, start + i as usize, hash);
        }

        let mut min = 0;
        for i in 1..=nums[start] {
            if let Some(x) = hash.get(&(start + i as usize)) {
                if min == 0 || 1 + *x < min {
                    min = 1 + *x;
                }
            }
        }
        if min != 0 {
            hash.insert(start, min);
        }
    }

    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut end, mut max_position, mut steps) = (0, 0, 0);

        for i in 0..nums.len() - 1 {
            max_position = max_position.max(nums[i] + i as i32);
            if i == end {
                end = max_position as usize;
                steps += 1;
            }
        }

        steps
    }
}
