///! 根据src/bin中新加的题目生成相应的markdown文件
use leetcode;
use leetcode::write_to_readme;

fn main() {
    // 获取新加的文件
    let files = leetcode::get_new_file_in_bin();

    for i in files.iter() {
        let res = leetcode::get_question_msg(i);
        write_to_readme(res);
    }
}
