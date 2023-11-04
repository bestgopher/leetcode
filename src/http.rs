use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

lazy_static! {
    static ref RE: Regex = Regex::new(r".*?/problems/(.*?)/").unwrap();
}

const URL: &str = "https://leetcode-cn.com/graphql/";

#[derive(Debug, Eq, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn new(s: &str) -> Self {
        match s {
            "Easy" => Self::Easy,
            "Medium" => Self::Medium,
            "Hard" => Self::Hard,
            _ => Self::Easy,
        }
    }
}

impl Serialize for Difficulty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Easy => serializer.serialize_str("Easy"),
            Self::Medium => serializer.serialize_str("Medium"),
            Self::Hard => serializer.serialize_str("Hard"),
        }
    }
}

impl<'de> Deserialize<'de> for Difficulty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DifficultyVisitor;

        impl<'de> Visitor<'de> for DifficultyVisitor {
            type Value = Difficulty;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    "Easy" => Ok(Self::Value::Easy),
                    "Medium" => Ok(Self::Value::Medium),
                    "Hard" => Ok(Self::Value::Hard),
                    _ => Err(Error::unknown_variant(v, &["Easy", "Medium", "Hard"])),
                }
            }
        }

        deserializer.deserialize_str(DifficultyVisitor)
    }
}

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
    pub difficulty: Difficulty,
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

pub async fn get_question_info(mut ques: &str) -> Resp {
    if ques.starts_with("http") {
        ques = RE
            .captures_iter(ques)
            .next()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
    }

    let data_fmt = r#"{"operationName":"questionData","variables":{"titleSlug":"{}"},
    "query":"query questionData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    boundTopicId\n    title\n    titleSlug\n    content\n    translatedTitle\n    translatedContent\n    isPaidOnly\n    difficulty\n    likes\n    dislikes\n    isLiked\n    similarQuestions\n    contributors {\n      username\n      profileUrl\n      avatarUrl\n      __typename\n    }\n    langToValidPlayground\n    topicTags {\n      name\n      slug\n      translatedName\n      __typename\n    }\n    companyTagStats\n    codeSnippets {\n      lang\n      langSlug\n      code\n      __typename\n    }\n    stats\n    hints\n    solution {\n      id\n      canSeeDetail\n      __typename\n    }\n    status\n    sampleTestCase\n    metaData\n    judgerAvailable\n    judgeType\n    mysqlSchemas\n    enableRunCode\n    envInfo\n    book {\n      id\n      bookName\n      pressName\n      source\n      shortDescription\n      fullDescription\n      bookImgUrl\n      pressImgUrl\n      productUrl\n      __typename\n    }\n    isSubscribed\n    isDailyQuestion\n    dailyRecordStatus\n    editorType\n    ugcQuestionId\n    style\n    __typename\n  }\n}\n"}"#;

    let data = data_fmt.replace("{}", ques);

    Client::new()
        .post(URL)
        .header("content-type", "application/json")
        .body(data)
        .send()
        .await
        .unwrap()
        .json::<Resp>()
        .await
        .expect(format!("{} download failed", ques).as_str())
}

#[cfg(test)]
mod tests {
    use super::get_question_info;

    #[tokio::test]
    async fn test_get_question_info() {
        println!("{:?}", get_question_info("container-with-most-water").await);
        println!(
            "{:?}",
            get_question_info("https://leetcode-cn.com/problems/container-with-most-water/").await
        );
    }
}
