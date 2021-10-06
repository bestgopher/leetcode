fn main() {
    assert_eq!("202".to_string(), Solution::convert_to_base7(100));
    assert_eq!("-10".to_string(), Solution::convert_to_base7(-7));
}

struct Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let (mut s, mut num) = (String::new(), num);
        if num < 0 {
            s.push('-');
            num *= -1;
        }
        let mut index = 1;
        let mut s1 = 0;
        while num > 0 {
            s1 += (num % 7) * index;

            num /= 7;
            index *= 10;
        }

        s.push_str(s1.to_string().as_str());

        s
    }
}
