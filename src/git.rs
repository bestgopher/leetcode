use git2::{Repository, StatusOptions};

use std::process::Command;

/// 把新文件加入到git中
pub fn push() {
    // 解析readme.md文件
    let mut r = crate::file::parse_readme();
    let new_file = get_uncommit_files();
    for i in new_file.iter() {
        let x = i.trim_end_matches(".rs"); // 去掉后缀
        let x = x.trim_start_matches("src/bin/"); // 去掉路径
        git_add(i);
        r.push(crate::http::get_question_info(x));
    }

    crate::file::write_readme(&mut r);
    git_add("README.md");
    push_to_origin();
}

fn get_uncommit_files() -> Vec<String> {
    let mut options = StatusOptions::new();
    options.pathspec("src/bin");
    options.include_untracked(true);
    let repo = Repository::open(".").unwrap();
    let statuses = repo.statuses(Some(&mut options)).unwrap();

    statuses
        .iter()
        .map(|x| String::from(x.path().unwrap()))
        .collect()
}

fn git_add(file: &str) {
    Command::new("git").arg("add").arg(file).output().unwrap();
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(file)
        .output()
        .unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());
}

pub fn push_to_origin() {
    let output = Command::new("git").arg("push").output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::git::get_uncommit_files;

    #[test]
    fn test_get_uncommit_files() {
        println!("{:?}", get_uncommit_files());
    }
}
