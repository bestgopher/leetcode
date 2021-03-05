fn main() {}

struct Solution;

impl Solution {
    /// 常规解法，将数字转化为字符串，然后挨个比较
    pub fn is_palindrome1(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if x < 10 {
            return true;
        }

        let s = x.to_string().into_bytes();
        let (mut i1, mut i2) = (0, s.len() - 1);

        while i1 < i2 {
            if s[i1] != s[i2] {
                return false;
            }
            i1 += 1;
            i2 -= 1;
        }

        true
    }
    ///第二个想法是将数字本身反转，然后将反转后的数字与原始数字进行比较，如果它们是相同的，那么这个数字就是回文。
    /// 但是，如果反转后的数字大于 \text{int.MAX}int.MAX，我们将遇到整数溢出问题。
    ///
    /// 按照第二个想法，为了避免数字反转可能导致的溢出问题，为什么不考虑只反转 \text{int}int 数字的一半？毕竟，如果该数字是回文，其后半部分反转后应该与原始数字的前半部分相同。
    ///
    /// 例如，输入 1221，我们可以将数字 “1221” 的后半部分从 “21” 反转为 “12”，并将其与前半部分 “12” 进行比较，因为二者相同，我们得知数字 1221 是回文。
    pub fn is_palindrome(x: i32) -> bool {
        // 所有小于10的都是
        if x < 10 && x >= 0 {
            return true;
        }

        // 所有个位数有0的，即可以被10整除的都不是，例如1110
        if x < 0 || x % 10 == 0{
            return false;
        }

        let mut x = x;
        let mut r = 0;

        while x > r {
            r = x % 10 + r * 10;
            x /= 10;
        }

        // 数字为12321时，r 为123， x为12，因此r要处以10
        x == r || r / 10 == x
    }
}
