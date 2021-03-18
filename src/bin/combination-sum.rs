fn main() {}

struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::calc(&candidates, target)
    }

    fn calc(candidates: &Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut r = vec![];

        for &i in candidates.iter() {
            let mut v = vec![];
            if i == target {
                v.push(i);
                r.push(v);
            } else if i < target {
                let mut x = Self::calc(&candidates, target - i);
                for mut m in x {
                    if !m.is_empty() {
                        if i >= *m.last().unwrap() {  // 递增排列，用于去重复
                            m.push(i);
                            r.push(m);
                        }
                    }
                }
            }
        }
        r
    }
}
