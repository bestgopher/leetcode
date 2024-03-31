#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 栈
    /// 当栈顶出现两个#，则说明当前节点叶子节点，可以把叶子节点去掉(转化为一个#继续遍历)
    pub fn is_valid_serialization1(preorder: String) -> bool {
        let mut stack = vec![];
        for i in preorder.split(',') {
            match i {
                "#" => {
                    stack.push("#");
                    while stack.len() >= 3
                        && stack[stack.len() - 1] == "#"
                        && stack[stack.len() - 2] == "#"
                        && stack[stack.len() - 3] != "#"
                    {
                        stack.pop();
                        stack.pop();
                        stack.pop();
                        stack.push("#");
                    }
                }
                i => stack.push(i),
            }
        }

        stack.len() == 1 && stack[0] == "#"
    }

    /// 入度出度。入度为多少节点指向它，出度是指它指向多少节点
    /// 入度+1，出度-1，最终和为0 则表示有效
    /// 注：整个过程中和必须大于等于0
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut diff = 1;
        for i in preorder.split(',') {
            diff -= 1;
            if diff < 0 {
                return false;
            }

            if i != "#" {
                diff += 2;
            }
        }

        diff == 0
    }
}
