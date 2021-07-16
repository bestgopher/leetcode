fn main() {
    println!("{}", Solution::h_index(vec![3, 0, 6, 1, 5]));
    println!("{}", Solution::h_index(vec![1]));
}

struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort();

        let mut h = 0;

        for i in (0..citations.len()).rev() {
            if citations[i] > h {
                h += 1;
            }
        }

        return h;
    }
}
