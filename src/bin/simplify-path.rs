fn main() {}

struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut v = vec![];

        for i in path.split('/').into_iter() {
            if i != "" && i != "." {
                match i {
                    ".." => {
                        v.pop();
                    }
                    _ => v.push(i),
                }
            }
        }
        let mut s = String::new();
        if v.is_empty() {
            s.push('/');
        } else {
            for i in v.into_iter() {
                s.push('/');
                s.push_str(i);
            }
        }

        s
    }
}
