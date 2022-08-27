#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

enum Status {
    Start,
    PreBlank,            // 前空格
    Sign,                // 符号位
    Integer,             // 数字
    PointWithInteger,    // 前面带数字的小数点
    PointWithoutInteger, // 前面不带数字的小数点
    IntegerAfterPoint,   // 小数点后面的数字
    E,                   // E or e
    IntegerAfterE,       // E后面的数字
    SignAfterE,          // E 后面的符号
    LastInteger,         // 最后的数字
    PostBlank,           // 后空格
    Invalid,             // 无效的
}

impl Status {
    fn new() -> Status {
        Status::Start
    }

    fn transform(&mut self, x: u8) {
        use Status::*;

        match self {
            Start => match x {
                b' ' => *self = PreBlank,
                b'+' | b'-' => *self = Sign,
                b'0'..=b'9' => *self = Integer,
                b'.' => *self = PointWithoutInteger,
                _ => *self = Invalid,
            },
            PreBlank => match x {
                b' ' => *self = PreBlank,
                b'+' | b'-' => *self = Sign,
                b'0'..=b'9' => *self = Integer,
                b'.' => *self = PointWithoutInteger,
                _ => *self = Invalid,
            },
            Sign => match x {
                b'0'..=b'9' => *self = Integer,
                b'.' => *self = PointWithoutInteger,
                _ => *self = Invalid,
            },
            Integer => match x {
                b' ' => *self = PostBlank,
                b'0'..=b'9' => *self = Integer,
                b'.' => *self = PointWithInteger,
                b'E' | b'e' => *self = E,
                _ => *self = Invalid,
            },
            PointWithInteger => match x {
                b' ' => *self = PostBlank,
                b'0'..=b'9' => *self = IntegerAfterPoint,
                b'E' | b'e' => *self = E,
                _ => *self = Invalid,
            },
            PointWithoutInteger => match x {
                b'0'..=b'9' => *self = IntegerAfterPoint,
                _ => *self = Invalid,
            },
            IntegerAfterPoint => match x {
                b' ' => *self = PostBlank,
                b'0'..=b'9' => *self = IntegerAfterPoint,
                b'E' | b'e' => *self = E,
                _ => *self = Invalid,
            },
            E => match x {
                b'0'..=b'9' => *self = IntegerAfterE,
                b'+' | b'-' => *self = SignAfterE,
                _ => *self = Invalid,
            },
            IntegerAfterE => match x {
                b' ' => *self = PostBlank,
                b'0'..=b'9' => *self = IntegerAfterE,
                b'+' | b'-' => *self = SignAfterE,
                _ => *self = Invalid,
            },
            SignAfterE => match x {
                b'0'..=b'9' => *self = LastInteger,
                _ => *self = Invalid,
            },
            LastInteger => match x {
                b' ' => *self = PostBlank,
                b'0'..=b'9' => *self = LastInteger,
                _ => *self = Invalid,
            },
            PostBlank => match x {
                b' ' => *self = PostBlank,
                _ => *self = Invalid,
            },
            Invalid => {}
        }
    }

    fn is_valid(&self) -> bool {
        use Status::*;
        match self {
            Integer | PointWithInteger | IntegerAfterPoint | LastInteger | PostBlank
            | IntegerAfterE => true,
            _ => false,
        }
    }
}

struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut status = Status::new();
        for &i in s.as_bytes() {
            status.transform(i);
        }

        status.is_valid()
    }
}
