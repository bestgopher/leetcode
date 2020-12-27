fn main() {
    println!("{}", Solution::freq_alphabets("10#11#12".to_string()));
    println!("{}", Solution::freq_alphabets("1326#".to_string()));
}

struct Solution;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut result = String::new();

        let mut index = 0;
        let bytes = s.as_bytes();

        while index < bytes.len() {
            if index + 2 < bytes.len() && bytes[index + 2] == '#' as u8 {
                result.push((96 + (bytes[index]-48)* 10 + (bytes[index + 1]-48)) as char);
                index += 3;
            } else {
                result.push((96 + bytes[index]-48) as char);
                index += 1;
            }
        }
        result
    }
}