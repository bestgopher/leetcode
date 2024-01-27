#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_number_of_alloys(
        n: i32,
        k: i32,
        budget: i32,
        composition: Vec<Vec<i32>>,
        stock: Vec<i32>,
        cost: Vec<i32>,
    ) -> i32 {
        (0..k as usize)
            .map(|i| {
                // 已有的能制造多少
                let mut x = (0..n as usize)
                    .map(|x| stock[x] / composition[i][x])
                    .min()
                    .unwrap();

                let mut total_cost = 0; // 购买的总消费
                loop {
                    let mut p = 0;

                    for m in 0..n as usize as usize {
                        // 表示剩下已有的不够造了，因此需要去买
                        if stock[m] - x * composition[i][m] <= 0 {
                            p += composition[i][m] * cost[m];
                        } else {
                            if stock[m] - x * composition[i][m] < composition[i][m] {
                                p += (composition[i][m] - stock[m] + x * composition[i][m])
                                    * cost[m];
                            }
                        }
                    }

                    if p + total_cost <= budget {
                        x += 1;
                        total_cost += p;
                    } else {
                        break;
                    }
                }

                x
            })
            .max()
            .unwrap()
    }
}
