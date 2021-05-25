fn main() {}

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut start, mut end) = (1, numbers.len());

        while start < end {
            let sum = numbers[start - 1] + numbers[end - 1];
            match target.cmp(&sum) {
                std::cmp::Ordering::Equal => break,
                std::cmp::Ordering::Less => end -= 1,
                std::cmp::Ordering::Greater => start += 1,
            }
        }

        vec![start as i32, end as i32]
    }
}
