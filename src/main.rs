///! 根据src/bin中新加的题目生成相应的markdown文件
use leetcode;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        git();
    } else if args.len() == 2 {
        new(args[1].as_str());
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

fn new(filename: &str) {
    let resp = leetcode::get_question_msg(filename);
    leetcode::make_new_file(resp);
}