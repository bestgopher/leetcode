fn main() {
    assert_eq!(
        "012".to_string(),
        Solution::original_digits("owoztneoer".to_string())
    );
    assert_eq!(
        "0123456789".to_string(),
        Solution::original_digits("zeroonetwothreefourfivesixseveneightnine".to_string())
    );
}

struct Solution;

impl Solution {
    // zero
    // one
    // two
    // three
    // four
    // five
    // six
    // seven
    // eight
    // nine
    // efghinorstuvwxz
    pub fn original_digits(s: String) -> String {
        let mut l = [0; 26];
        let mut count = [0; 10]; // 用于记录每个数字的出现次数
        for &i in s.as_bytes().iter() {
            l[(i - 97) as usize] += 1;
        }

        let mut s = String::new();

        for &i in "zwuxghfsin".as_bytes().iter() {
            if i == b'z' {
                count[0] += l[(i - 97) as usize]
            } else if i == b'w' {
                count[2] += l[(i - 97) as usize]
            } else if i == b'u' {
                count[4] += l[(i - 97) as usize]
            } else if i == b'x' {
                count[6] += l[(i - 97) as usize]
            } else if i == b'g' {
                count[8] += l[(i - 97) as usize]
            } else if i == b'h' {
                count[3] += l[(i - 97) as usize] - count[8] // 三的数量是h的数量剪掉8的数量
            } else if i == b'f' {
                count[5] += l[(i - 97) as usize] - count[4]
            } else if i == b's' {
                count[7] += l[(i - 97) as usize] - count[6]
            } else if i == b'i' {
                count[9] += l[(i - 97) as usize] - count[6] - count[5] - count[8]
            } else if i == b'n' {
                count[1] += l[(i - 97) as usize] - count[7] - count[9] * 2 // 因为nine是两个n
            }
        }

        for (i, &v) in count.iter().enumerate() {
            if v != 0 {
                s.push_str(i.to_string().repeat(v).as_str());
            }
        }

        s
    }
}
