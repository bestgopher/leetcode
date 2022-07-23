#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(
        39,
        Solution::subarray_sum(vec![0, 0, 0, 0, 1, -1, 0, 0, 1, 1, 0, 0, 2, -2, 2], 2)
    );
}

struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sums = std::collections::HashMap::<i32, i32>::new();
        sums.insert(0, 1);
        let mut sum = 0;
        let mut count = 0;
        for &i in nums.iter() {
            sum += i;
            if let Some(s) = sums.get(&(sum - k)) {
                count += *s
            }
            sums.entry(sum).and_modify(|x| *x += 1).or_insert(1);
        }
        count
    }
}
