fn main() {}

struct Solution;

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut v = String::new();
        let mut start = (0i32, 0i32);
        for i in target.as_bytes().iter().map(|x| (*x) as i32) {
            let t = ((i - 97) / 5, (i - 97) % 5);
            let interval = (t.0 - start.0, t.1 - start.1);

            if interval.0 > 0 {
                v.push_str("D".repeat((interval.0 - t.0 / 5) as usize).as_str())
            } else {
                v.push_str("U".repeat((-1 * interval.0) as usize).as_str())
            }

            if interval.1 > 0 {
                v.push_str("R".repeat(interval.1 as usize).as_str())
            } else {
                v.push_str("L".repeat((-1 * interval.1) as usize).as_str())
            }

            if i == b'z' as i32 {
                if interval.0 != 0 && t.0 / 5 > 0 {
                    v.push('D');
                }
            }

            v.push('!');

            start = t;
        }

        v
    }
}
