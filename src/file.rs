use lazy_static::lazy_static;
use regex::Regex;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio_stream::wrappers::ReadDirStream;
use tokio_stream::StreamExt;

use crate::http::{Data, Difficulty, Ques, Resp};

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"\|\s*([0-9]*)\s*\|\s*(.*?)\s*\|.*?bin/(.*?)\.rs.*?\|.*?\|\s*?(\w*)\s*?\|")
            .unwrap();
}

/// 将结果写入README.md中
pub async fn write_readme(r: &mut Vec<Resp>) {
    // 先按id排序
    r.sort_by(|x, y| {
        let x_id = x.data.question.question_id.parse::<i32>().unwrap();
        let y_id = y.data.question.question_id.parse::<i32>().unwrap();
        x_id.cmp(&y_id)
    });
    let s = crate::render::render(r).unwrap();

    match tokio::fs::write("README.md", s).await {
        Ok(_) => (),
        Err(e) => eprintln!("写入 README.md 失败，err{}", e.to_string()),
    }
}

/// 获取 src/bin 目录下所有文件的名称
pub async fn get_all_bin_file() -> Vec<String> {
    let mut dir = ReadDirStream::new(fs::read_dir("src/bin/").await.unwrap());
    let mut v = vec![];
    while let Some(x) = dir.next().await {
        v.push(
            x.unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .trim_end_matches(".rs")
                .to_string(),
        );
    }

    v
}

/// 创建 bin/{quest_name}.rs 文件
pub async fn write_question(resp: Resp) {
    let file = format!("src/bin/{}.rs", resp.data.question.title_slug);
    if std::path::Path::new(file.as_str()).exists() {
        eprintln!("{} exists", file);
        return;
    }

    let mut f = File::create(file.as_str()).await.unwrap();
    let mut s = String::new();
    s.push_str("#![allow(dead_code, unused, unused_variables, non_snake_case)]\n\n");
    s.push_str("fn main() {}\n\n");
    s.push_str("struct Solution;\n\n");

    for i in resp.data.question.code_snippets {
        if i.lang == "Rust" {
            s.push_str(i.code.replace("↵", "\n").as_str());
            s.push('\n');
            break;
        }
    }

    f.write_all(s.as_bytes()).await.unwrap();
}

/// 解析README.md
pub async fn parse_readme() -> Vec<Resp> {
    let contents = fs::read_to_string("README.md").await.unwrap();
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
                        difficulty: Difficulty::new(i.get(4).unwrap().as_str()),
                    },
                },
            })
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::get_all_bin_file;
    use super::parse;

    #[tokio::test]
    async fn test_parse_readme() {
        let contents = tokio::fs::read_to_string("README.md").await.unwrap();
        let x = parse(&contents);
        println!("{:?}", x);
        println!("{}", x.len());

        for i in x {
            println!(
                "{}, {}, {}, {:?}",
                i.data.question.translated_title,
                i.data.question.question_id,
                i.data.question.title_slug,
                i.data.question.difficulty
            );
        }
    }

    #[tokio::test]
    async fn test_get_all_bin_file() {
        println!("{:?}", get_all_bin_file().await);
    }
}
