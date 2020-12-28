fn main() {
    assert_eq!(true, Solution::is_perfect_square(4), "4");
    assert_eq!(false, Solution::is_perfect_square(5), "5");
    assert_eq!(false, Solution::is_perfect_square1(14), "14");
    assert_eq!(true, Solution::is_perfect_square(16), "16");
    assert_eq!(false, Solution::is_perfect_square(2147483647), "2147483647");
    assert_eq!(true, Solution::is_perfect_square(100000000), "100000000");
    assert_eq!(true, Solution::is_perfect_square1(49000000), "49000000");
}


struct Solution;

impl Solution {
    // 使用二分的思维，
    // 从1开始尝试，
    // 当基数的平方小于num时，基数再次扩大2倍
    // 当基数的平凡大于num时，则基数处于当前基数与上一个基数之间，取中间数尝试
    // 当上一个基数与当前基数相等，且基数的平方不为num时，说明此数不是完全平方数，否则是完全平方数
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 && num == 0 {
            return true;
        }

        let mut num1 = 1;
        let mut last_num = 1;

        loop {
            if num / num1 == num1 {
                return if num % num1 == 0 {
                    true
                } else {
                    false
                };
            } else if num / num1 > num1 {
                last_num = num1;
                num1 *= 2;
            } else {
                num1 = (last_num + num1) / 2;
            }

            if num1 == last_num && num / num1 != num1 {
                return false;
            }
        }

        false
    }

    // 利用1=1， 4=1+3， 9=1+3+5， 16=1+3+5+7，。。。。
    // (n+1)**2 - n**2 = 2n+1
    pub fn is_perfect_square1(num: i32) -> bool {
        let mut start = 1;
        let mut sum = num;
        loop {
            if sum ==  0 {
                return true;
            } else if sum < 0 {
                return false;
            } else {
                sum -= start;
            }
            start += 2;
        }
    }
}
