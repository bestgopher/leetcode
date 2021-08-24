fn main() {
    println!("{}", Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]));
    println!("{}", Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 5, 6]));
    println!("{}", Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 10, 11, 12, 13]));
}

struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 { return 0; }

        let mut count = 0;
        let mut num = 2;
        let mut s = nums[1] - nums[0];

        for i in 2..nums.len() {
            if nums[i] - nums[i - 1] == s {
                num += 1;
            } else {
                if num >= 3 { count += (1..=num - 3 + 1).into_iter().sum::<i32>(); }
                s = nums[i] - nums[i - 1];
                num = 2;
            }
        }

        if num >= 3 { count += (1..=num - 3 + 1).into_iter().sum::<i32>(); }

        count
    }
}
