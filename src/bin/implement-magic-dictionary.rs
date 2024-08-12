#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */

#[derive(Clone)]
struct Tier {
    data: Vec<Option<Box<Tier>>>,
    is_end: bool,
}

impl Tier {
    fn new() -> Self {
        Tier {
            data: vec![None; 26],
            is_end: false,
        }
    }

    fn insert(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        let mut node = self;

        for i in data {
            let index = (i - b'a') as usize;
            if node.data[index].is_none() {
                node.data[index] = Some(Box::new(Tier::new()));
            }
            node = node.data[index].as_mut().unwrap();
        }

        node.is_end = true;
    }

    /// flag为true表示已经变换过
    fn search(&self, data: &[u8], flag: bool) -> bool {
        if data.is_empty() {
            return self.is_end && flag;
        }
        let index = (data[0] - b'a') as usize;
        if let Some(x) = self.data[index].as_ref() {
            if x.search(&data[1..], flag | false) {
                return true;
            }
        }

        if !flag {
            for i in (0..26).filter(|&x| x != index) {
                if let Some(x) = self.data[i].as_ref() {
                    if x.search(&data[1..], true) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

struct MagicDictionary {
    tier: Tier,
}

impl MagicDictionary {
    fn new() -> Self {
        Self { tier: Tier::new() }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        dictionary
            .into_iter()
            .for_each(|x| self.tier.insert(x.as_bytes()));
    }

    fn search(&self, search_word: String) -> bool {
        self.tier.search(search_word.as_bytes(), false)
    }
}
