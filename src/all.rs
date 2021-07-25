use crate::file;

use std::sync::{Arc, Mutex};
use std::thread;
use crate::http::Resp;


/// 重新格式化
pub fn all() {
    let files = file::get_all_bin_file();

    let mut v = Vec::<Resp>::with_capacity(files.len());

    let x = Arc::new(Mutex::new(v));
    let mut handlers = vec![];

    for i in 0..=files.len() / 10 {
        // 把files分块，分成10个文件一块
        let files = if i * 10 + 10 > files.len() {
            files[i * 10..files.len()].to_vec()
        } else {
            files[i * 10..i * 10 + 10].to_vec()
        };

        let x = x.clone();

        handlers.push(thread::spawn(move || {
            for i in files {
                let resp = crate::http::get_question_info(&i);
                x.lock().unwrap().push(resp);
            }
        }))
    }

    for i in handlers {
        i.join();
    }

    crate::file::write_readme(&mut *x.lock().unwrap());
}
