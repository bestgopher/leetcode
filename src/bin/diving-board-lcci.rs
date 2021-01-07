fn main() {}

struct Solution;

impl Solution {
    pub fn diving_board(shorter: i32, longer: i32, k: i32) -> Vec<i32> {
        let mut v = Vec::<i32>::new();
        if k == 0 {
            return v;
        }

        for i in 0..=k {
            let s = i * longer + (k - i) * shorter;
            if v.is_empty() {
                v.push(r);
            }

            if s != v[v.len() - 1] {
                v.push(r);
            }
        }

        v
    }
}
