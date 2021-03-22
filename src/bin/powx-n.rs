fn main() {
    println!("{}", Solution::my_pow(0.00001, 2147483647));
    println!("{}", Solution::my_pow(2.00000, 10));
    println!("{}", Solution::my_pow(2.00000, /**/-2));
    println!("{}", Solution::my_pow(2.00000, -2147483648));
}

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0f64 {
            return 0f64;
        }

        if n == 0 {
            return 1f64;
        }

        let mut n = n;

        let flag = n < 0;
        let mut result = x;
        let mut n1 = 2i32;
        let s = n == std::i32::MIN;
        if s {
            n += 1;
        }

        while n.abs() > 1 {
            result *= result;
            let n2 = n1.overflowing_mul(2);
            if n2.1 || n2.0 > n.abs() {
                result *= Self::my_pow(x, n.abs() - n1);
                break;
            }
            n1 = n2.0;
        }

        if s { result *= x }

        if flag { 1f64 / result } else { result }
    }
}
