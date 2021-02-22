fn main() {
    assert_eq!(
        "Dire".to_string(),
        Solution::predict_party_victory("DDRRR".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut senate = senate
            .as_bytes()
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<u8>>();

        let (mut r_ban, mut d_ban) = (0, 0);

        loop {
            let (mut r_num, mut d_num) = (0, 0);
            for i in 0..senate.len() {
                if senate[i] == b'x' {
                    continue;
                }
                if senate[i] == b'R' {
                    if r_ban > 0 {
                        r_ban -= 1;
                        senate[i] = b'x';
                    } else {
                        d_ban += 1;
                        r_num += 1;
                    }
                } else {
                    if d_ban > 0 {
                        d_ban -= 1;
                        senate[i] = b'x';
                    } else {
                        r_ban += 1;
                        d_num += 1;
                    }
                }
            }

            if r_num == 0 {
                return "Dire".to_string();
            }

            if d_num == 0 {
                return "Radiant".to_string();
            }
        }
    }
}
