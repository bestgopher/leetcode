fn main() {}

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![];
        }

        if nums.len() == 1 {
            return vec![nums];
        }
        let mut result = Vec::with_capacity(Self::factorial(nums.len()));
        for i in 0..nums.len() {
            let mut v = Vec::with_capacity(nums.len() - 1);
            v.extend_from_slice(&nums[..i]);
            v.extend_from_slice(&nums[i + 1..]);
            let r = Self::permute(v);

            for j in r.into_iter() {
                let mut s = Vec::with_capacity(nums.len() - 1);
                s.push(nums[i]);
                s.extend_from_slice(&j);
                result.push(s);
            }
        }

        result
    }

    fn factorial(x: usize) -> usize {
        if x == 1 {
            return 1usize;
        }

        x * Self::factorial(x - 1)
    }
}
