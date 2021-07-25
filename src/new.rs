/// 1.获取question name
/// 2.如果name是url，就不需要拼装，不是就需要拼装
/// 3.请求结构，获取数据
/// 4.将数据写入bin/{question_name}.rs文件中
pub fn new(ques: String) {
    let r = crate::http::get_question_info(ques.as_str());
    crate::file::write_question(r);
}
