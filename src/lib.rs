#[cfg(test)]
mod tests {
    use crate::{get_new_file_in_bin, get_question_msg, get_question_num};
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_get_new_file_in_bin() {
        println!("{:?}", get_new_file_in_bin());
    }

    #[test]
    fn test_get_question_num() {
        assert_eq!(13usize, get_question_num());
    }

    #[test]
    fn test_write_to_readme() {
        let resp = get_question_msg("maximum-points-you-can-obtain-from-cards");
        write_to_readme(resp);
    }

    #[test]
    fn test_get_question_no() {
        assert_eq!(374, get_question_no("- 374：猜数字大小"));
        assert_eq!(367, get_question_no("- 367：有效的完全平方数"));
    }

    #[test]
    fn test_get_question_msg() {
        println!("{:?}", get_question_msg("maximum-points-you-can-obtain-from-cards"));
    }

    #[test]
    fn test_make_new_file() {
        make_new_file(get_question_msg("shu-de-zi-jie-gou-lcof"));
    }
}

extern crate reqwest;

use git2::{Repository, StatusOptions};
use serde::Deserialize;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;


/// 获取bin目录下新加的文件
pub fn get_new_file_in_bin() -> Vec<String> {
    let mut options = StatusOptions::new();
    options.include_untracked(true);
    let repo = Repository::open(".").unwrap();
    let statuses = repo.statuses(Some(&mut options)).unwrap();

    statuses.iter().
        filter(|x| { x.path().unwrap().starts_with("src/bin/") }).
        map(|x| String::from(x.path().unwrap())).
        map(|x| {
            let x = x.trim_end_matches(".rs");  // 去掉路径
            let x = x.trim_start_matches("src/bin/");  // 去掉后缀
            x.to_string()
        }).
        collect()
}

#[derive(Deserialize, Debug)]
pub struct Ques {
    #[serde(rename = "questionId")]
    question_id: String,
    #[serde(rename = "titleSlug")]
    title_slug: String,
    #[serde(rename = "translatedTitle")]
    translated_title: String,
    #[serde(rename = "codeSnippets")]
    code_snippets: Vec<CodeSnippets>,
}

#[derive(Deserialize, Debug)]
pub struct CodeSnippets {
    #[serde(rename = "code")]
    code: String,
    #[serde(rename = "lang")]
    lang: String,
    #[serde(rename = "langSlug")]
    lang_slug: String,
    #[serde(rename = "__typename")]
    typename: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    question: Ques,
}

#[derive(Deserialize, Debug)]
pub struct Resp {
    data: Data,
}

/// 通过名字获取题目的ID
pub fn get_question_msg(name: &str) -> Resp {
    let url = "https://leetcode-cn.com/graphql/";
    let data_fmt = r#"{"operationName":"questionData","variables":{"titleSlug":"{}"},"query":"query questionData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    boundTopicId\n    title\n    titleSlug\n    content\n    translatedTitle\n    translatedContent\n    isPaidOnly\n    difficulty\n    likes\n    dislikes\n    isLiked\n    similarQuestions\n    contributors {\n      username\n      profileUrl\n      avatarUrl\n      __typename\n    }\n    langToValidPlayground\n    topicTags {\n      name\n      slug\n      translatedName\n      __typename\n    }\n    companyTagStats\n    codeSnippets {\n      lang\n      langSlug\n      code\n      __typename\n    }\n    stats\n    hints\n    solution {\n      id\n      canSeeDetail\n      __typename\n    }\n    status\n    sampleTestCase\n    metaData\n    judgerAvailable\n    judgeType\n    mysqlSchemas\n    enableRunCode\n    envInfo\n    book {\n      id\n      bookName\n      pressName\n      source\n      shortDescription\n      fullDescription\n      bookImgUrl\n      pressImgUrl\n      productUrl\n      __typename\n    }\n    isSubscribed\n    isDailyQuestion\n    dailyRecordStatus\n    editorType\n    ugcQuestionId\n    style\n    __typename\n  }\n}\n"}"#;
    let data = data_fmt.replace("{}", name);
    let res = reqwest::blocking::Client::new().
        post(url).
        header("content-type", "application/json").
        body(data).
        send().
        unwrap().
        json::<Resp>().unwrap();

    res
}

/// 把问题写到README.md中
pub fn write_to_readme(question_info: Resp) {
    let readme = fs::read_to_string("README.md").unwrap();
    let mut write_string = String::new();
    write_string.push_str("# leetcode
通过rust刷leetcode题目。
通过刷leetcode题目学习rust。\n\n");
    write_string.push_str(format!("当前已刷：{}\n\n", get_question_num()).as_str());
    write_string.push_str("### 题目\n");

    let mut f = File::create("README.md").unwrap();

    let mut index = 0usize;
    let split = readme.split("\n").into_iter().collect::<Vec<&str>>();
    let mut flag = false;
    let mut flag1 = false;
    let no = question_info.data.question.question_id.parse::<i32>().unwrap();
    while index + 3 < split.len() {
        if !flag {
            if split[index] == "### 题目" {
                flag = true;
            }
            index += 1;
            continue;
        }

        let i1 = get_question_no(split[index]);

        if i1 == no {
            continue;
        }

        if !flag1 && i1 > no {
            flag1 = true;
            write_string.push_str(format!("- {}：{}\n", no, question_info.data.question.translated_title).as_str());
            write_string.push_str(format!("    - [src](https://github.com/rustors/leetcode/blob/main/src/bin/{}.rs)\n", question_info.data.question.title_slug).as_str());
            write_string.push_str(format!("    - [leetcode](https://leetcode-cn.com/problems/{}/)\n", question_info.data.question.title_slug).as_str());
        }

        write_string.push_str(format!("{}\n{}\n{}\n", split[index], split[index + 1], split[index + 2]).as_str());
        index += 3;
    }

    if !flag1 {
        write_string.push_str(format!("- {}：{}\n", no, question_info.data.question.translated_title).as_str());
        write_string.push_str(format!("    - [src](https://github.com/rustors/leetcode/blob/main/src/bin/{}.rs)\n", question_info.data.question.title_slug).as_str());
        write_string.push_str(format!("    - [leetcode](https://leetcode-cn.com/problems/{}/)\n", question_info.data.question.title_slug).as_str());
    }

    let _ = f.write(write_string.as_bytes());
}

/// 获取题目总数
fn get_question_num() -> usize {
    let dir = fs::read_dir("src/bin/").unwrap();

    dir.
        into_iter().
        filter(|x| {
            if let Ok(f) = x {
                f.file_name().to_str().unwrap().ends_with(".rs")
            } else {
                false
            }
        }).count()
}

/// 获取题目的编号
fn get_question_no(s: &str) -> i32 {
    s.split("：")
        .into_iter()
        .collect::<Vec<&str>>()[0]
        .trim_end_matches('：')
        .trim_start_matches('-')
        .trim_start()
        .parse::<i32>()
        .unwrap()
}

/// git add
pub fn git_add_commit_files(files: Vec<String>) {
    for file in files.iter() {
        add_and_commit(format!("src/bin/{}.rs", file).as_str()); // 将新加的文件提交到git仓库
    }

    add_and_commit("README.md");  // 将修改的README.md提交到git仓库
    push_to_origin();  // 将文件
}

fn add_and_commit(file: &str) {
    Command::new("git").arg("add").arg(file).output().unwrap();
    Command::new("git").arg("commit").arg("-m").arg(file).output().unwrap();
}

fn push_to_origin() {
    Command::new("git").arg("push").output().unwrap();
}

/// 通过题目名获取rust答题模板
pub fn make_new_file(resp: Resp) {
    let mut f = File::create(format!("src/bin/{}.rs", resp.data.question.title_slug)).unwrap();
    let mut s = String::new();
    s.push_str("fn main() {}\n\nstruct Solution;\n\n");

    for i in resp.data.question.code_snippets {
        if i.lang == "Rust" {
            s.push_str(i.code.replace("↵", "\n").as_str());
            s.push_str("\n");
            break;
        }
    }

    f.write(s.as_bytes()).unwrap();
}
