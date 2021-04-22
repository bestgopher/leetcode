fn main() {
    println!("{:?}", Solution::gray_code(2));
    println!("{:?}", Solution::gray_code(0));
    println!("{:?}", Solution::gray_code(3));
}

struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(2i32.pow(n as u32) as usize);
        result.push(0);
        for i in 0..n as usize {
            let x = result.len() - 1;
            for j in 0..2usize.pow(i as u32) {
                result.push(result[x - j] | (1 << i));
            }
        }

        result
    }
}
