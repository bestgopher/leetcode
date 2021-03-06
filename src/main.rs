///! 根据src/bin中新加的题目生成相应的markdown文件
use leetcode;
use leetcode::{get_question_msg, write_to_readme};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        git();
    } else if args.len() == 2 {
        if args[1].as_str() == "-a" {
            all();
        } else {
            new(args[1].as_str());
        }
    } else {
        panic!("只能有一个参数");
    }
}

fn git() {
    // 获取新加的文件
    let files = leetcode::get_new_file_in_bin();

    for i in files.iter() {
        let res = leetcode::get_question_msg(i);
        leetcode::write_to_readme(res);
    }

    leetcode::git_add_commit_files(files);
}

fn all() {
    let files = std::fs::read_dir("./src/bin").unwrap();
    let files = files
        .into_iter()
        .filter(|x| {
            if let Ok(f) = x {
                f.file_name().to_str().unwrap().ends_with(".rs")
            } else {
                false
            }
        })
        .map(|x| {
            x.unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .trim_end_matches(".rs")
                .trim_start_matches("src/bin/")
                .to_string()
        })
        .collect::<Vec<String>>();
    files.iter().for_each(|x| {
        println!("{} start", x);
        let resp = get_question_msg(x);
        write_to_readme(resp);
    });

    leetcode::add_and_commit("README.md");
    leetcode::push_to_origin();
}

fn new(filename: &str) {
    let resp = leetcode::get_question_msg(filename);
    leetcode::make_new_file(resp);
}
