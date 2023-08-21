#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let (start, target) = (start.as_bytes(), target.as_bytes());
        let (mut i, mut j) = (0, 0);

        while i < start.len() && j < target.len() {
            while i < start.len() && start[i] == b'_' {
                i += 1;
            }

            while j < target.len() && target[j] == b'_' {
                j += 1;
            }

            if i >= start.len() || j >= target.len() {
                break;
            }

            if start[i] != target[j] {
                return false;
            }

            match start[i] {
                b'L' if i < j => return false,
                b'R' if i > j => return false,
                _ => {
                    i += 1;
                    j += 1;
                }
            }
        }

        while i < start.len() {
            if start[i] != b'_' {
                return false;
            }
            i += 1;
        }

        while j < target.len() {
            if target[j] != b'_' {
                return false;
            }
            j += 1;
        }

        true
    }
}
