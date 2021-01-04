fn main() {
    assert_eq!(2, Solution::is_prefix_of_word("this problem is an easy problem".to_string(), "pro".to_string()));
    assert_eq!(4, Solution::is_prefix_of_word("i use triple pillow".to_string(), "pill".to_string()));
    assert_eq!(-1, Solution::is_prefix_of_word("hello from the other side".to_string(), "they".to_string()));
    assert_eq!(-1, Solution::is_prefix_of_word("hellohello hellohellohello".to_string(), "ell".to_string()));
}

struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let mut index = 0;
        let mut word = 0;
        let mut flag = true;
        for &i in sentence.as_bytes().iter() {
            if flag && i == search_word.as_bytes()[index] {
                if index == search_word.len() - 1 {
                    return word + 1 as i32;
                }
                index += 1;
                continue;
            }

            if i == b' ' {
                word += 1;
                flag = true;
            } else {
                flag = false;
            }
            index = 0;
        }

        -1
    }
}
