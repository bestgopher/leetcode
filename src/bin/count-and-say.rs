#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!("1211".to_string(), Solution::count_and_say(4));
}

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let mut n = n;
        let mut array = vec![1; 1];
        let mut index = 0;

        while n > 1 {
            let start = array.len();
            let (mut num, mut x) = (1, array[index]);
            for i in index + 1..array.len() {
                if array[i] == x {
                    num += 1;
                } else {
                    array.push(num);
                    array.push(x);
                    num = 1;
                    x = array[i];
                }
            }
            array.push(num);
            array.push(x);
            index = start;
            n -= 1;
        }

        let mut result = String::new();

        for i in index..array.len() {
            result.push_str(array[i].to_string().as_str());
        }
        result
    }
}
