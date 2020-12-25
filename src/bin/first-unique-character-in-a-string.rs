fn main() {}

struct Solution;

impl Solution {
    /// 因为都是小写字母，所以最多只有26种字符，
    /// 申请一个26位长的数组，循环遍历字符串，每个字符的下标对应位字符-'a'，
    /// 每次循环，数组中对应下标的值+1；遍历字符串后，再遍历字符串一次，
    /// 找到字符对应下标的值，值为1则说明只存在一次。因此对应的下标则为所求。
    pub fn first_uniq_char(s: String) -> i32 {
        let mut a = [0; 26];
        let a_char = 'a' as usize;

        for &i in s.as_bytes().iter() {
            a[(i as usize) - a_char] += 1;
        }

        for (i, &value) in s.as_bytes().iter().enumerate() {
            if a[(value as usize) - a_char] == 1 {
                return i as i32;
            }
        }
        -1
    }
}
