fn main() {
    // assert_eq!(Solution::convert("LEETCODEISHIRINGS".to_string(), 4), "LDREOEIIECIHNSTSG".to_string());
    assert_eq!(Solution::convert("AB".to_string(), 1), "AB".to_string());
}

struct Solution;

impl Solution {
    /// s = "LEETCODEISHIRINGS", numRows = 4
    /// 输出: "LDREOEIIECIHNSTSG"
    /// L     D     R
    /// E   O E   I I
    /// E C   I H   N S
    /// T     S     G
    ///
    /// s = "LEETCODEISHIRINGS", numRows = 5
    /// 输出："LISEESGEDHNTOIICR"
    ///
    /// L     I    S
    /// E   E S   G
    /// E  D  H  N
    /// T O   I I
    /// C     R
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let s = s.chars().into_iter().collect::<Vec<char>>();
        let mut s1 = Vec::<char>::with_capacity(s.len());
        let num = num_rows as usize * 2 - 2; // 把v字形看成一组。则num代表一组的成员

        for i in 0..=num_rows as usize - 1 {
            let mut index = i as usize;
            while index < s.len() {
                s1.push(s[index]);
                index += num;
                if i != 0 && i != num_rows as usize - 1 && index - 2 * i < s.len() {
                    s1.push(s[index - 2 * i]);
                }
            }
        }

        s1.into_iter().collect::<String>()
    }
}
