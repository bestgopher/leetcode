fn main() {}

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut l = Self::s(&nums[..]);
        l.push(vec![]);
        l
    }

    fn s(nums: &[i32]) -> Vec<Vec<i32>> {
        let mut r = vec![];
        if nums.len() == 0 {
            return r;
        }

        let x = nums[0];
        r.push(vec![x]);
        for j in Self::s(&nums[1..]) {
            let mut m = Vec::with_capacity(1 + j.len());
            m.push(x);
            m.extend_from_slice(&j[..]);
            r.push(j);
            r.push(m);
        }

        r
    }
}
