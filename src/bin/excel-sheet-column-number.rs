fn main() {
    println!("{}", Solution::title_to_number("AB".to_string()));
    println!("{}", Solution::title_to_number("ZY".to_string()));
}

struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut r = 0;

        for &i in column_title.as_bytes() {
            r += r * 25 + (i - b'A') as i32 + 1;
        }

        r
    }
}
