fn main() {
    assert_eq!(
        vec!["Alaska".to_string(), "Dad".to_string()],
        Solution::find_words(vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string()
        ])
    );
}

struct Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let v = vec![(); 10];
        let m1: std::collections::HashMap<_, _> = "qwertyuiop"
            .as_bytes()
            .into_iter()
            .map(|x| *x)
            .zip(v.iter())
            .collect();
        let m2: std::collections::HashMap<_, _> = "asdfghjkl"
            .as_bytes()
            .into_iter()
            .map(|x| *x)
            .zip(v.iter())
            .collect();
        let m3: std::collections::HashMap<_, _> = "zxcvbnm"
            .as_bytes()
            .into_iter()
            .map(|x| *x)
            .zip(v.iter())
            .collect();

        let mut result = vec![];

        for i in words.iter() {
            let mut s = i.as_bytes()[0];
            if s < 97 {
                s += 32;
            }
            let m = if let Some(_) = m1.get(&s) {
                &m1
            } else if let Some(_) = m2.get(&s) {
                &m2
            } else {
                &m3
            };
            let mut flag = true;
            for j in i.as_bytes() {
                if let (None, None) = (m.get(j), m.get(&(*j + 32))) {
                    flag = false;
                    break;
                }
            }

            if flag {
                result.push(i.clone())
            }
        }

        result
    }
}
