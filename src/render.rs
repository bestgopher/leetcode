use lazy_static::lazy_static;
use tera::{Tera, Context, Error as TeraError, ErrorKind, Result as TeraResult};

/// 模板内容
const TEMPLATE_STR: &'static str = r"# leetcode

当前已刷：{{ datas | length }}

### 题目

| 编号 | 题目  | 描述                                                            | 代码                                                  |
| ---- | ----- | ------------------------------------------------------------ | ----------------------------------------------------- |
{% for t in datas %}|{{ t.data.question.questionId }}    | {{ t.data.question.translatedTitle }} | [src](https://github.com/rustors/leetcode/blob/main/src/bin/{{ t.data.question.titleSlug }}.rs) | [leetcode](https://leetcode-cn.com/problems/{{ t.data.question.titleSlug }}) |
{% endfor %}";

static A: &str = "";

/// 模板名字
const TEMPLATE_NAME: &'static str = "leetcode";

lazy_static!(
    /// 用于渲染的模板
    pub static ref TEMPLATE: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_template(TEMPLATE_NAME, TEMPLATE_STR).unwrap();
        tera
    };
);


/// 把传入的内容渲染为模板内容
pub fn render(data: &Vec<crate::http::Resp>) -> TeraResult<String> {
    let mut ctx = Context::new();
    ctx.insert("datas", data);

    TEMPLATE.render(TEMPLATE_NAME, &ctx)
}

#[cfg(test)]
mod tests {
    use crate::http::{Resp, Ques, Data};
    use crate::render::render;

    #[test]
    fn test_render() {
        let data = vec![
            Resp {
                data: Data {
                    question: Ques {
                        question_id: "111".to_string(),
                        title_slug: "aaa".to_string(),
                        translated_title: "中国".to_string(),
                        code_snippets: vec![],
                        difficulty: String::new();
                    }
                },
            },
        ];

        println!("{:?}", render(&data).unwrap().to_string());
    }
}