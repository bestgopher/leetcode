fn main() {
    println!("{:?}", Solution::restore_ip_addresses(String::from("25525511135")));
    println!("{:?}", Solution::restore_ip_addresses(String::from("0000")));
    println!("{:?}", Solution::restore_ip_addresses(String::from("010010")));
    println!("{:?}", Solution::restore_ip_addresses(String::from("1111")));
    println!("{:?}", Solution::restore_ip_addresses(String::from("0279245587303")));
}

struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        Self::ip(&s, 4)
    }

    fn ip(s: &str, level: i32) -> Vec<String> {
        let mut result = vec![];

        if s.len() < level as usize {
            return result;
        }

        if level == 1 {
            // 这里故意设置为256，使得数字大于255，为true
            return if (s.starts_with('0') && s.len() > 1) || s.parse::<i32>().unwrap_or(256) > 255 {
                result
            } else {
                vec![String::from(s)]
            };
        }


        for i in 0..3 {
            if s.len() > i {
                if i == 2 && s[..3].parse::<i32>().unwrap_or(256) > 255 {
                    return result;
                }

                let r = Self::ip(&s[i + 1..], level - 1);
                for j in r {
                    result.push(format!("{}.{}", &s[0..=i], j));
                }

                if s.starts_with('0') {
                    return result;
                }
            }
        }

        result
    }
}
