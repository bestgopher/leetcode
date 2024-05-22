#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut winners = std::collections::HashSet::new();
        let mut losers = std::collections::HashMap::new();

        for i in matches {
            losers.entry(i[1]).and_modify(|x| *x += 1).or_insert(1);
            winners.remove(&i[1]);
            if !losers.contains_key(&i[0]) {
                winners.insert(i[0]);
            }
        }

        let mut w: Vec<i32> = winners.into_iter().collect();
        let mut l: Vec<i32> = losers
            .into_iter()
            .filter_map(|x| if x.1 == 1 { Some(x.0) } else { None })
            .collect();

        w.sort_unstable();
        l.sort_unstable();

        vec![w, l]
    }
}
