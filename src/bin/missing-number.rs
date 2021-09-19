fn main() {
    println!("{}", Solution::missing_number1(vec![3, 0, 1]));
    println!(
        "{}",
        Solution::missing_number1(vec![9, 6, 4, 2, 3, 5, 7, 0, 1])
    );
}

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (0..=nums.len() as i32).sum::<i32>() - nums.into_iter().sum::<i32>()
    }

    pub fn missing_number1(nums: Vec<i32>) -> i32 {
        let l = nums.len() as i32;
        nums.into_iter()
            .enumerate()
            .fold(l, |l, (x, y)| x as i32 ^ y ^ l)
    }
}
