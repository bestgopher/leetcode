#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    println!(
        "{}",
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1])
    );

    println!(
        "{}",
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2])
    );
}

struct Solution;

impl Solution {
    pub fn validate_stack_sequences1(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let (mut index1, mut index2) = (0, 0);
        let mut stack = vec![];

        while index1 < pushed.len() || index2 < pushed.len() {
            if index1 < pushed.len()
                && (stack.is_empty() || *stack.last().unwrap() != popped[index2])
            {
                stack.push(pushed[index1]);
                index1 += 1;
            } else {
                match stack.pop() {
                    Some(x) if x == popped[index2] => {
                        index2 += 1;
                    }
                    _ => return false,
                }
            }
        }

        stack.is_empty()
    }

    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut index = 0;
        let mut stack = vec![];

        for i in pushed {
            stack.push(i);

            while let Some(&x) = stack.last() {
                if x == popped[index] {
                    stack.pop();
                    index += 1;
                } else {
                    break;
                }
            }
        }

        stack.is_empty()
    }
}
