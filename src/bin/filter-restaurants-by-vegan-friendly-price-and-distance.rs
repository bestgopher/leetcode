#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        use std::cmp::Ordering;

        let mut r = restaurants
            .into_iter()
            .filter(|x| vegan_friendly == 0 || x[2] == vegan_friendly)
            .filter(|x| x[3] <= max_price)
            .filter(|x| x[4] <= max_distance)
            .map(|x| (x[0], x[1]))
            .collect::<Vec<(i32, i32)>>();

        r.sort_by(|x, y| {
            if x.1 == y.1 {
                y.0.cmp(&x.0)
            } else {
                y.1.cmp(&x.1)
            }
        });

        r.into_iter().map(|(x, _)| x).collect()
    }
}
