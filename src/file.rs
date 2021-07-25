use std::fs::{self, File};
use std::io::Write;
use lazy_static::lazy_static;
use regex::Regex;

use crate::http::{Resp, Data, Ques};

lazy_static!(
    static ref RE: Regex = Regex::new(r"\|\s*([0-9]*)\s*\|\s*(.*?)\s*\|.*?bin/(.*?)\.rs.*?\|.*?\|").unwrap();
);

/// 将结果写入README.md中
pub fn write_readme(r: &mut Vec<Resp>) {
    // 先按id排序
    r.sort_by(|x, y| {
        let x_id = x.data.question.question_id.parse::<i32>().unwrap();
        let y_id = y.data.question.question_id.parse::<i32>().unwrap();
        x_id.cmp(&y_id)
    });
    let s = crate::render::render(r).unwrap();

    match std::fs::write("README.md", s) {
        Ok(_) => (),
        Err(e) => println!("写入 README.md 失败，err{}", e.to_string())
    }
}

/// 获取 src/bin 目录下所有文件的名称
pub fn get_all_bin_file() -> Vec<String> {
    let dir = fs::read_dir("src/bin/").unwrap();
    dir.
        into_iter().
        map(|x| {
            x.unwrap().file_name().to_str().unwrap().trim_end_matches(".rs").to_string()
        }).
        collect()
}


/// 创建 bin/{quest_name}.rs 文件
pub fn write_question(resp: Resp) {
    let file = format!("src/bin/{}.rs", resp.data.question.title_slug);
    if std::path::Path::new(file.as_str()).exists() {
        println!("{} exists", file);
        return;
    }

    let mut f = File::create(file.as_str()).unwrap();
    let mut s = String::new();
    s.push_str("fn main() {}\n\nstruct Solution;\n\n");

    for i in resp.data.question.code_snippets {
        if i.lang == "Rust" {
            s.push_str(i.code.replace("↵", "\n").as_str());
            s.push('\n');
            break;
        }
    }

    f.write_all(s.as_bytes()).unwrap();
}

/// 解析README.md
pub fn parse_readme() -> Vec<Resp> {
    let contents = fs::read_to_string("README.md").unwrap();
    parse(&contents)
}

fn parse(contents: &str) -> Vec<Resp> {
    let mut v = vec![];
    for content in contents.split('\n') {
        for i in RE.captures_iter(content.trim()) {
            v.push(Resp {
                data: Data {
                    question: Ques {
                        question_id: i.get(1).unwrap().as_str().to_string(),
                        translated_title: i.get(2).unwrap().as_str().to_string(),
                        title_slug: i.get(3).unwrap().as_str().to_string(),
                        code_snippets: vec![],
                        difficulty: String::new(),
                    }
                },
            })
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use crate::file::{parse, get_all_bin_file};

    #[test]
    fn test_parse_readme() {
        let contents = std::fs::read_to_string("README.md").unwrap();
        let x = parse(&contents);
        println!("{:?}", x);
        println!("{}", x.len());

        for i in x {
            println!("{}, {}, {}", i.data.question.translated_title, i.data.question.question_id, i.data.question.title_slug);
        }
    }

    #[test]
    fn test_get_all_bin_file() {
        println!("{:?}", get_all_bin_file());
    }
}