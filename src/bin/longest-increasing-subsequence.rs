fn main() {
    println!(
        "{}",
        Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18])
    );
    println!("{}", Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));
    println!("{}", Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]));
}

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut v = vec![1; nums.len()];

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    v[i] = v[i].max(v[j] + 1);
                }
            }
        }

        v.into_iter().fold(0, |x, y| x.max(y))
    }
}
