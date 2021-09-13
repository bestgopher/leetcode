fn main() {
    println!("{}", Solution::find_nth_digit(1));
    println!("{}", Solution::find_nth_digit(2));
    println!("{}", Solution::find_nth_digit(10));
    println!("{}", Solution::find_nth_digit(11));
    println!("{}", Solution::find_nth_digit(12));
    println!("{}", Solution::find_nth_digit(13));
    println!("{}", Solution::find_nth_digit(14));
    println!("{}", Solution::find_nth_digit(190));
    println!("{}", Solution::find_nth_digit(191));
    println!("{}", Solution::find_nth_digit(192));
    println!("{}", Solution::find_nth_digit(193));
    println!("{}", Solution::find_nth_digit(312313));
    println!("{}", Solution::find_nth_digit(1000000000));
}

struct Solution;

impl Solution {
    /// 1.先找到n代表的数字有几位，规律：数字每增加一位，个数为：1*9 + 20*9 + 300*9...
    pub fn find_nth_digit(n: i32) -> i32 {
        if n <= 9 {
            return n;
        }

        let mut n = n;

        let mut s: i32 = 1;  // n所在的数字有几位

        loop {
            let m = 9i64 * s as i64 * 10i64.pow(s as u32 - 1) as i64;
            if (n as i64) < m {
                break;
            } else {
                n -= m as i32;
                s += 1;
            }
        }

        // 基于相同位数的最小值的偏移，比如1123相对于1000偏移123
        let n1 = n - 1;


        let x = 10i32.pow(s as u32 - 1) + n1 / s;  // 定位到具体的数字

        x / 10i32.pow((s - n1 % s - 1) as u32) % 10
    }
}
