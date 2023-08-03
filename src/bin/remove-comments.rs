#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        enum Type {
            /// 没有注释
            NoComment,
            /// 块注释
            Comment1,
            /// 行注释
            Comment2,
        }

        let mut s = vec![];

        let mut new_line = Vec::new();
        let mut t = Type::NoComment;

        for old_line in source {
            let mut i = 0;
            while i < old_line.len() {
                match t {
                    Type::NoComment => {
                        if old_line[i..].starts_with("/*") {
                            t = Type::Comment1;
                            i += 1;
                        } else if old_line[i..].starts_with("//") {
                            t = Type::Comment2;
                            i += 1;
                        } else {
                            new_line.push(old_line.as_bytes()[i]);
                        }
                    }
                    Type::Comment1 => {
                        if old_line[i..].starts_with("*/") {
                            i += 1;
                            t = Type::NoComment;
                        }
                    }
                    Type::Comment2 => {
                        t = Type::NoComment;
                        break;
                    }
                }
                i += 1;
            }

            if !matches!(t, Type::Comment1) && !new_line.is_empty() {
                s.push(unsafe { String::from_utf8_unchecked(new_line) });
                new_line = Vec::new();
            }
        }

        s
    }
}
