mod new;
mod http;
mod file;
mod git;
mod render;
mod all;

use clap::{App, Arg};

use std::process;

pub fn run() {
    let matches = App::new("leetcode")
        .version("0.0.1")
        .author("bestgopher <84328409@qq.com>")
        .about("a helper for leetcode")
        .subcommand(App::new("new")
            .about("get a new leetcode question")
            .arg(Arg::new("question_name")
                .about("The configuration file to use")
                .index(1)))
        .subcommand(App::new("all")
            .about("get all questions' info and rewrite README.md"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        match matches.value_of_t::<String>("question_name") {
            Ok(x) => new::new(x),
            Err(_) => {
                eprintln!("please input the name of question");
                process::exit(1);
            }
        }
    } else if matches.subcommand_matches("all").is_some() {
        all::all();
    } else {
        git::push();
    }
}
