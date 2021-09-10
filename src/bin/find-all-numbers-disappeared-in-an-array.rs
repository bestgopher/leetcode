fn main() {}

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers1(nums: Vec<i32>) -> Vec<i32> {
        let mut s = std::collections::HashMap::new();

        for &i in nums.iter() {
            s.insert(i, ());
        }

        let mut v = vec![];
        for i in 1..=nums.len() as i32 {
            if s.get(&i).is_none() {
                v.push(i);
            }
        }

        v
    }

    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let l = nums.len();
        for i in 0..l {
            let v = nums[i] - 1;
            nums[v as usize % l] += l as i32;
        }

        nums
            .into_iter()
            .enumerate()
            .filter(|&(_, x)| x <= (l as i32))
            .map(|x| x.0 as i32 + 1)
            .collect()
    }
}
