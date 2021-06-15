fn main() {}

struct Solution;

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
struct WordDictionary {
    root: Vec<Option<Box<Node>>>,
}

struct Node {
    table: Vec<Option<Box<Node>>>,
    // 到此节点是否为一个单词
    is_word: bool,
}

impl Node {
    fn new(is_word: bool) -> Self {
        let mut table = Vec::with_capacity(26);
        for _i in 0..26 {
            table.push(None);
        }

        Self {
            table,
            is_word,
        }
    }
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        let mut root = Vec::with_capacity(26);
        for _i in 0..26 {
            root.push(None);
        }

        Self {
            root
        }
    }

    fn add_word(&mut self, word: String) {
        Self::add(&mut self.root[..], word.as_bytes());
    }

    fn add(table: &mut [Option<Box<Node>>], letter: &[u8]) {
        if letter.len() == 0 {
            return;
        }

        let is_word = letter.len() == 1;

        let s = if letter[0] == b'.' { b'a'..=b'z' } else { letter[0]..=letter[0] };

        for i in s {
            let mut node = table[(i - b'a') as usize].as_mut();
            if node.is_none() {
                table[(i - b'a') as usize] = Some(Box::new(Node::new(is_word)));
            } else {
                node.as_mut().unwrap().is_word = is_word;
            }

            Self::add(table[(i - b'a') as usize].as_mut().unwrap().table.as_mut(), &letter[1..]);
        }
    }

    fn search(&self, word: String) -> bool {
        Self::search_by_slice(&self.root[..], word.as_bytes())
    }

    fn search_by_slice(table: &[Option<Box<Node>>], letter: &[u8]) -> bool {
        if letter.len() == 0 {
            return false;
        }

        let s = if letter[0] == b'.' { b'a'..=b'z' } else { letter[0]..=letter[0] };
        for i in s {
            let node = table[(i - b'a') as usize].as_ref();
            if node.is_none() {
                continue;
            }

            if letter.len() == 1 {
                if node.as_ref().unwrap().is_word {
                    return true;
                }
            }

            if Self::search_by_slice(node.as_ref().unwrap().table.as_ref(), &letter[1..]) {
                return true;
            }
        }

        false
    }
}
