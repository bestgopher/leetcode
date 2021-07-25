use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};


lazy_static! {
    static ref RE: Regex = Regex::new(r".*?/problems/(.*?)/").unwrap();
}

const URL: &'static str = "https://leetcode-cn.com/graphql/";

#[derive(Deserialize, Serialize, Debug)]
pub struct Ques {
    #[serde(rename = "questionId")]
    pub question_id: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    #[serde(rename = "translatedTitle")]
    pub translated_title: String,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<CodeSnippets>,
    #[serde(rename = "difficulty")]
    pub difficulty: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeSnippets {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub question: Ques,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Resp {
    pub data: Data,
}

pub fn get_question_info(mut ques: &str) -> Resp {
    if ques.starts_with("http") {
        ques = RE.captures_iter(ques).next().unwrap().get(1).unwrap().as_str();
    }

    let data_fmt = r#"{"operationName":"questionData","variables":{"titleSlug":"{}"},
    "query":"query questionData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    boundTopicId\n    title\n    titleSlug\n    content\n    translatedTitle\n    translatedContent\n    isPaidOnly\n    difficulty\n    likes\n    dislikes\n    isLiked\n    similarQuestions\n    contributors {\n      username\n      profileUrl\n      avatarUrl\n      __typename\n    }\n    langToValidPlayground\n    topicTags {\n      name\n      slug\n      translatedName\n      __typename\n    }\n    companyTagStats\n    codeSnippets {\n      lang\n      langSlug\n      code\n      __typename\n    }\n    stats\n    hints\n    solution {\n      id\n      canSeeDetail\n      __typename\n    }\n    status\n    sampleTestCase\n    metaData\n    judgerAvailable\n    judgeType\n    mysqlSchemas\n    enableRunCode\n    envInfo\n    book {\n      id\n      bookName\n      pressName\n      source\n      shortDescription\n      fullDescription\n      bookImgUrl\n      pressImgUrl\n      productUrl\n      __typename\n    }\n    isSubscribed\n    isDailyQuestion\n    dailyRecordStatus\n    editorType\n    ugcQuestionId\n    style\n    __typename\n  }\n}\n"}"#;
    let data = data_fmt.replace("{}", ques);

    Client::new()
        .post(URL)
        .header("content-type", "application/json")
        .body(data)
        .send()
        .unwrap()
        .json::<Resp>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::http::get_question_info;

    #[test]
    fn test_get_question_info() {
        println!("{:?}", get_question_info("container-with-most-water"));
        println!("{:?}", get_question_info("https://leetcode-cn.com/problems/container-with-most-water/"));
    }
}
