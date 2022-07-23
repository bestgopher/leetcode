#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(
        "6".to_string(),
        Solution::multiply("2".to_string(), "3".to_string())
    );
    assert_eq!(
        "56088".to_string(),
        Solution::multiply("123".to_string(), "456".to_string())
    );
    assert_eq!(
        "998001".to_string(),
        Solution::multiply("999".to_string(), "999".to_string())
    );
    assert_eq!("30501687172287445993560048081057096686019986405658336826483685740920318317486606305094807387278589614".to_string(), Solution::multiply("60974249908865105026646412538664653190280198809433017".to_string(), "500238825698990292381312765074025160144624723742".to_string()));
}

struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if &num1 == "0" || &num2 == "0" {
            return "0".to_string();
        }

        let mut res = vec![0i32; num1.len() + num2.len()];

        for (i1, &v1) in num1.as_bytes().iter().enumerate() {
            for (i2, &v2) in num2.as_bytes().iter().enumerate() {
                let s = (v1 - b'0') as i32 * (v2 - b'0') as i32;
                res[i1 + i2 + 1] += s % 10;
                res[i1 + i2] += s / 10;
            }
        }

        let mut s = vec!["-".to_string(); res.len()];
        let (mut i, len) = (0, res.len() - 1);
        while i <= len {
            if res[len - i] < 10 {
                s[len - i] = res[len - i].to_string();
            } else {
                s[len - i] = (res[len - i] % 10).to_string();
                res[len - i - 1] += res[len - i] / 10;
            }
            i += 1;
        }
        s.join("").trim_start_matches('0').to_string()
    }
}
