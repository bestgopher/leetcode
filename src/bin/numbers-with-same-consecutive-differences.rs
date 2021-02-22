fn main() {}

struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut v = vec![];
        for i in 1..=9 {
            let mut stack = vec![];
            stack.push(i.to_string());
            while let Some(elem) = stack.pop() {
                if elem.len() == n as usize {
                    v.push(elem.parse::<i32>().unwrap());
                } else {
                    let s = elem.as_str()[elem.len() - 1..].parse::<i32>().unwrap();
                    if s + k < 10 {
                        let mut e = elem.clone();
                        e.push_str((s + k).to_string().as_str());
                        stack.push(e);
                    }

                    if k != 0 && s - k >= 0 {
                        let mut e = elem.clone();
                        e.push_str((s - k).to_string().as_str());
                        stack.push(e);
                    }
                }
            }
        }

        v
    }
}
