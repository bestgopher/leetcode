#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut hash = std::collections::HashMap::new();
        let mut s = (lo..=hi).collect::<Vec<i32>>();
        s.sort_unstable_by(|x, y| {
            match Self::weight(*x, &mut hash).cmp(&Self::weight(*y, &mut hash)) {
                std::cmp::Ordering::Equal => x.cmp(y),
                m => m,
            }
        });

        s[k as usize - 1]
    }

    fn weight(mut num: i32, hash: &mut std::collections::HashMap<i32, i32>) -> i32 {
        let mut weight = 0;
        while num != 1 {
            if let Some(&x) = hash.get(&num) {
                weight += x;
                break;
            }

            if num % 2 == 0 {
                num /= 2;
            } else {
                num = 3 * num + 1;
            }

            weight += 1;
        }
        hash.insert(num, weight);
        weight
    }
}
