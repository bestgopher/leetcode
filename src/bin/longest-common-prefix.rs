fn main() {
    assert_eq!(
        "fl".to_string(),
        Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ])
    );
    assert_eq!(
        "".to_string(),
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ])
    );
}

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let (mut result, mut index) = (Vec::new(), 0usize);
        loop {
            if index >= strs[0].len() {
                break;
            }
            let i = strs[0].as_bytes()[index];
            let mut flag = false;

            for j in strs.iter() {
                if index >= j.len() || j.as_bytes()[index] != i {
                    flag = true;
                    break;
                }
            }

            if flag {
                break;
            }

            result.push(i);
            index += 1;
        }

        String::from_utf8(result).unwrap()
    }
}
