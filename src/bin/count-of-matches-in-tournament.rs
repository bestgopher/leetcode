fn main() {
    assert_eq!(6, Solution::number_of_matches(7));
}

struct Solution;

impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut count = 0;

        while n > 0 {
            if n % 2 == 0 {
                count += n / 2;
                n = n / 2;
            } else {
                if n == 1 {
                    n = 0;
                } else {
                    count += n / 2;
                    n = n / 2 + 1;
                }
            }
        }

        count
    }
    /// 找规律，就n-1
    pub fn number_of_matches1(n: i32) -> i32 {
        n - 1
    }
}
