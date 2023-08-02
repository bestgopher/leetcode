#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 将较大的放在前面，然后找出后面最小且不在前面的数
    pub fn flipgame(mut fronts: Vec<i32>, mut backs: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();

        for i in 0..fronts.len() {
            if fronts[i] == backs[i] {
                set.insert(fronts[i]);
            }
        }

        let mut r = -1;

        for i in fronts.into_iter().chain(backs.into_iter()) {
            if !set.contains(&i) {
                if r == -1 {
                    r = i;
                } else {
                    r = r.min(i);
                }
            }
        }

        r.max(0)
    }
}
