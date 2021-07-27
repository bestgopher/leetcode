fn main() {
    assert_eq!(3, Solution::min_steps(3));
    assert_eq!(7, Solution::min_steps(10));
    assert_eq!(7, Solution::min_steps(7));
    assert_eq!(8, Solution::min_steps(18));
    assert_eq!(23, Solution::min_steps(23));
}

struct Solution;

impl Solution {
    pub fn min_steps(mut n: i32) -> i32 {
        if n == 1 { return 0; }

        // 检查是否是质数
        let mut count = 0;
        loop {
            let mut flag = false;
            for i in 2..n / 2 {
                if n % i == 0 {
                    count += i;
                    n = n / i;
                    flag = true;
                    break;
                }
            }
            if !flag {
                count += n;
                break;
            }
        }

        count
    }
}
