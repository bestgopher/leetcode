#![allow(dead_code, unused, unused_variables)]

fn main() {
    let s = Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1);
    println!("{s:?}");

    let s = Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2);
    println!("{s:?}");
}

struct Solution;

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let indeies: Vec<usize> = nums
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == key)
            .map(|(i, _)| i)
            .collect();

        let mut data = vec![];

        nums.into_iter().enumerate().for_each(|(index, _value)| {
            for &i in indeies.iter() {
                if (index > i && index - i <= k as usize) || (index <= i && i - index <= k as usize)
                {
                    data.push(index as i32);
                    break;
                }
            }
        });

        data.sort();

        data
    }
}
