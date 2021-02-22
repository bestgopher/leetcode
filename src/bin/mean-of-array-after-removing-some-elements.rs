fn main() {}

struct Solution;

impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort();

        let f = arr[(arr.len() as f64 * 0.05) as usize..(arr.len() as f64 * 0.95) as usize]
            .iter()
            .sum::<i32>() as f64
            / (arr.len() as f64 * 0.9);

        f
    }
}
