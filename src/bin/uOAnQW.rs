#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maxmium_score(cards: Vec<i32>, cnt: i32) -> i32 {
        let mut cards = cards;
        cards.sort_unstable_by(|x, y| x.cmp(y).reverse());

        let (mut min_even, mut min_odd) = (None::<i32>, None::<i32>);
        let (mut max_even, mut max_odd) = (None::<i32>, None::<i32>);
        let mut sum = 0;

        for &i in cards[0..cnt as usize].iter() {
            sum += i;
            if i % 2 == 0 {
                min_even = min_even.map_or(Some(i), |x| Some(x.min(i)));
            } else {
                min_odd = min_odd.map_or(Some(i), |x| Some(x.min(i)));
            }
        }

        if sum % 2 == 0 {
            return sum;
        }

        for &i in cards[cnt as usize..].iter() {
            if i % 2 == 0 {
                max_even = max_even.map_or(Some(i), |x| Some(x.max(i)));
            } else {
                max_odd = max_odd.map_or(Some(i), |x| Some(x.max(i)));
            }
        }

        match (min_even, max_odd, min_odd, max_even) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => (sum - x1 + y1).max(sum - x2 + y2),
            (Some(x1), Some(y1), _, _) => sum - x1 + y1,
            (_, _, Some(x2), Some(y2)) => sum - x2 + y2,
            _ => 0,
        }
    }
}
