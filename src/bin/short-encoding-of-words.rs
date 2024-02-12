#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::fmt::Debug;

fn main() {
    assert_eq!(
        Solution::minimum_length_encoding(vec![
            "time".to_string(),
            "me".to_string(),
            "bell".to_string(),
        ], ),
        10
    );

    assert_eq!(Solution::minimum_length_encoding(vec!["t".to_string()]), 2);
    assert_eq!(
        Solution::minimum_length_encoding(vec![
            "time".to_string(),
            "atime".to_string(),
            "btime".to_string(),
        ]),
        12
    );
    assert_eq!(
        Solution::minimum_length_encoding(vec!["me".to_string(), "time".to_string()]),
        5
    );
}

struct Solution;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort_unstable_by(|x, y| x.len().cmp(&y.len()).reverse());
        let mut set = std::collections::HashSet::<String>::new();

        'L: for i in words {
            for j in set.iter() {
                if j.ends_with(i.as_str()) {
                    continue 'L;
                }
            }

            set.insert(i);
        }

        set.iter().map(|x| x.len() as i32).sum::<i32>() + set.len() as i32
    }

    /// tire tree 字典树
    // pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
    //     struct TireTree {
    //         nodes: std::collections::HashMap<u8, TireTree>,
    //     }
    //
    //     impl TireTree {
    //         fn new() -> Self {
    //             TireTree {
    //                 nodes: std::collections::HashMap::new(),
    //             }
    //         }
    //
    //         // 如果为true则表示插入了新的
    //         fn insert(&mut self, word: &str) -> bool {
    //             if word.is_empty() {
    //                 return false;
    //             }
    //
    //             let k = word.as_bytes()[word.len() - 1];
    //
    //             if let Some(n) = self.nodes.get_mut(&k) {
    //                 n.insert(&word[..word.len() - 1])
    //             } else {
    //                 let mut n = TireTree::new();
    //                 let x = n.insert(&word[..word.len() - 1]);
    //                 self.nodes.insert(k, n);
    //                 true
    //             }
    //         }
    //     }
    //
    //     let mut tree = TireTree::new();
    //     let mut result = 0;
    //     for i in words.iter() {
    //         if tree.insert(i.as_str()) {
    //             result += i.len() as i32 + 1;
    //         }
    //     }
    //
    //     result
    // }
}
