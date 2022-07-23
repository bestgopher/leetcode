#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!(
        "{:?}",
        Solution::subdomain_visits(
            vec![
                "900 google.mail.com",
                "50 yahoo.com",
                "1 intel.mail.com",
                "5 wiki.org"
            ]
            .into_iter()
            .map(|x| x.to_string())
            .collect()
        )
    );
}

struct Solution;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut m = std::collections::HashMap::new();

        for i in cpdomains.iter() {
            let split = i.split(" ").collect::<Vec<&str>>();
            let num = split[0].parse::<i32>().unwrap();
            let url = split[1];

            m.entry(String::from(url))
                .and_modify(|x| *x += num)
                .or_insert(num);

            for (index, v) in url.as_bytes().iter().enumerate() {
                if *v == '.' as u8 {
                    m.entry(url[index + 1..].to_string())
                        .and_modify(|x| *x += num)
                        .or_insert(num);
                }
            }
        }

        m.iter().map(|(x, y)| format!("{} {}", y, x)).collect()
    }
}
