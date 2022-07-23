#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

struct Trie {
    /// 截止到此字母是否为一个单词
    is_word: bool,
    /// 树的节点
    nodes: std::collections::HashMap<u8, Trie>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            is_word: false,
            nodes: std::collections::HashMap::with_capacity(26),
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        self.insert_bytes(word.as_bytes());
    }

    fn insert_bytes(&mut self, words: &[u8]) {
        if words.len() < 1 {
            return;
        }

        let i = words[0];
        let is_last = words.len() == 1;
        if let Some(x) = self.nodes.get_mut(&i) {
            if is_last {
                x.is_word = true;
            }
            x.insert_bytes(&words[1..]);
        } else {
            let mut x = Self::new();
            if is_last {
                x.is_word = true;
            }
            x.insert_bytes(&words[1..]);
            self.nodes.insert(i, x);
        }
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        self.search_bytes(word.as_bytes(), false)
    }

    fn search_bytes(&self, words: &[u8], is_prefix: bool) -> bool {
        if words.len() == 0 {
            return is_prefix || self.is_word;
        }

        if let Some(x) = self.nodes.get(&words[0]) {
            x.search_bytes(&words[1..], is_prefix)
        } else {
            false
        }
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        self.search_bytes(prefix.as_bytes(), true)
    }
}

// /**
//  * Your Trie object will be instantiated and called as such:
//  * let obj = Trie::new();
//  * obj.insert(word);
//  * let ret_2: bool = obj.search(word);
//  * let ret_3: bool = obj.starts_with(prefix);
//  */
