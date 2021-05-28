fn main() {
    println!("{}", Solution::convert_to_title(701));
    println!("{}", Solution::convert_to_title(27));
    println!("{}", Solution::convert_to_title(28));
    println!("{}", Solution::convert_to_title(29));
    println!("{}", Solution::convert_to_title(26));
    println!("{}", Solution::convert_to_title(1));
    println!("{}", Solution::convert_to_title(26));
    println!("{}", Solution::convert_to_title(52));
}

struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut s = String::new();

        let mut column_number = column_number;
        while column_number > 0 {
            let pop = (column_number - 1) % 26;
            column_number = (column_number - 1) / 26;
            s.push(('A' as u8 + pop as u8) as char);
        }

        s.chars().rev().collect::<String>()
    }
}
