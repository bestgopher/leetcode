fn main() {}

struct Solution;

impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut s = std::collections::HashMap::<i32, ()>::with_capacity(b.len());
        let (a_sum, mut b_sum) = (a.iter().sum::<i32>(), 0);
        for i in b.into_iter() {
            s.insert(i, ());
            b_sum += i;
        }

        let mut result: Vec<i32> = Vec::with_capacity(2);

        for i in a.into_iter() {
            if let Some(_) = s.get(&(i - (a_sum - b_sum) / 2)) {
                result.push(i);
                result.push(i - (a_sum - b_sum) / 2);
                break;
            }
        }

        result
    }
}
