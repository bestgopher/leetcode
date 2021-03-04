fn main() {}

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        if a == b && a == "0".to_string() {
            return a;
        }

        let (a, b) = (a.as_bytes(), b.as_bytes());
        let mut s = vec![b' '; a.len().max(b.len()) + 1];
        let (mut m, mut index) = ((b'0', b'0'), 0);

        while a.len() >= index || b.len() >= index {
            let a1 = if a.len() >= index {
                a[a.len() - index - 1]
            } else {
                b'0'
            };

            let b1 = if b.len() >= index {
                b[b.len() - index - 1]
            } else {
                b'0'
            };


            m = Self::add(a1, b1, m.1);
            let l = s.len() - index - 1;
            s[l] = m.0;
            index += 1;
        }

        if m.1 == b'1' {
            let l = s.len() - index - 1;
            s[l] = b'1';
        }

        String::from_utf8(s).unwrap().trim_start_matches('0').to_string()
    }

    /// 返回a+b的和与进制
    fn add(a: u8, b: u8, c: u8) -> (u8, u8) {
        match (a, b, c) {
            (b'1', b'1', b'1') => (b'1', b'1'),
            (b'0', b'1', b'1') => (b'0', b'1'),
            (b'1', b'0', b'1') => (b'0', b'1'),
            (b'1', b'1', b'0') => (b'0', b'1'),
            (b'0', b'0', b'1') => (b'1', b'0'),
            (b'0', b'1', b'0') => (b'1', b'0'),
            (b'1', b'0', b'0') => (b'1', b'0'),
            (b'0', b'0', b'0') => (b'0', b'0'),
            _ => (b'0', b'0'),
        }
    }
}
