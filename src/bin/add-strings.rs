fn main() {}

struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut i = 0u8; // 进制
        let (num1, num2) = (num1.as_bytes(), num2.as_bytes());
        let (mut i1, mut i2) = (num1.len(), num2.len());

        let mut v = vec![0u8; i1.max(i2) + 1];
        let mut vindex = v.len() - 1;

        loop {
            let a = if i1 == 0 && i2 > 0 {
                i2 -= 1;
                num2[i2] - b'0' + i
            } else if i1 > 0 && i2 == 0 {
                i1 -= 1;
                num1[i1] - b'0' + i
            } else {
                i2 -= 1;
                i1 -= 1;
                num1[i1] + num2[i2] - b'0' - b'0' + i
            };

            if a >= 10 {
                v[vindex] = a - 10;
                i = 1;
            } else {
                v[vindex] = a;
                i = 0;
            }

            if i1 == 0 && i2 == 0 {
                break;
            }

            vindex -= 1;
        }

        if i != 0 {
            vindex -= 1;
            v[vindex] = 1;
        }

        v[vindex..]
            .iter()
            .fold(String::with_capacity((&v[vindex..]).len()), |mut x, y| {
                x.push_str(&*y.to_string());
                x
            })
    }
}
