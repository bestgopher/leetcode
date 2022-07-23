#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */
struct Codec {
    hash: std::collections::HashMap<u64, String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {
            hash: std::collections::HashMap::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, long_url: String) -> String {
        let mut s = std::collections::hash_map::DefaultHasher::new();
        <String as std::hash::Hash>::hash(&long_url, &mut s);
        let hash_code =
            <std::collections::hash_map::DefaultHasher as std::hash::Hasher>::finish(&s);
        self.hash.insert(hash_code, long_url);
        format!("http://tinyurl.com/{}", hash_code)
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        let hash_code = short_url
            .trim_start_matches("http://tinyurl.com/")
            .parse::<u64>()
            .unwrap();
        self.hash.get(&hash_code).unwrap().to_string()
    }
}
