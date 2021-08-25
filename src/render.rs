use lazy_static::lazy_static;
use tera::{Tera, Context, Result as TeraResult};

/// 模板内容
const TEMPLATE_STR: &str = r"# leetcode

| Total | Easy | Medium | Hard |
| :----: | :----: | :----: | :----: |
| {{ datas | length }} | {{ easy_num }} | {{ medium_num }} | {{ hard_num }} |

### 题目

| 编号 | 题目  | 代码    | 题目描述  |  难度 |
| ---- | ---- | ---- | ---- | ---- |
{% for t in datas %}|{{ t.data.question.questionId }}    | {{ t.data.question.translatedTitle }} | [src](https://github.com/rustors/leetcode/blob/main/src/bin/{{ t.data.question.titleSlug }}.rs) | [leetcode](https://leetcode-cn.com/problems/{{ t.data.question.titleSlug }}/) | {{ t.data.question.difficulty }} |
{% endfor %}";

/// 模板名字
const TEMPLATE_NAME: &str = "leetcode";

lazy_static!(
    /// 用于渲染的模板
    pub static ref TEMPLATE: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_template(TEMPLATE_NAME, TEMPLATE_STR).unwrap();
        tera
    };
);


/// 把传入的内容渲染为模板内容
pub fn render(data: &[crate::http::Resp]) -> TeraResult<String> {
    let mut ctx = Context::new();

    let easy_num = counter(data, |x| x.data.question.difficulty == crate::http::Difficulty::Easy);
    let medium_num = counter(data, |x| x.data.question.difficulty == crate::http::Difficulty::Medium);
    let hard_num = counter(data, |x| x.data.question.difficulty == crate::http::Difficulty::Hard);

    ctx.insert("datas", data);
    ctx.insert("easy_num", &easy_num);
    ctx.insert("medium_num", &medium_num);
    ctx.insert("hard_num", &hard_num);

    TEMPLATE.render(TEMPLATE_NAME, &ctx)
}

fn counter(data: &[crate::http::Resp], f: impl FnMut(&&crate::http::Resp) -> bool) -> usize {
    data.into_iter().filter(f).count()
}

#[cfg(test)]
mod tests {
    use crate::http::{Resp, Ques, Data, Difficulty};
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
                        difficulty: Difficulty::Easy,
                    }
                },
            },
        ];

        println!("{:?}", render(&data).unwrap().to_string());
    }
}
