fn main() {

}

struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut s = String::new();

        for (index, &c) in command.as_bytes().iter().enumerate() {
            if c == b'G' {
                s.push('G');
            } else if c == b'(' {
                if command.as_bytes()[index + 1] == b')' {
                    s.push('o')
                } else {
                    s.push_str("al");
                }
            }
        }
        s
    }
}
