fn main() {}

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut h = std::collections::HashMap::new();
        for i in magazine.bytes() {
            h.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        for i in ransom_note.bytes() {
            if let Some(x) = h.get_mut(&i) {
                if *x >= 1 {
                    *x -= 1;
                    continue;
                }
            }

            return false;
        }


        true
    }
}
