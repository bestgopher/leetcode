#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

struct Node {
    /// 入度个数
    indegrees: i32,
    /// 出度列表
    adjacency: Vec<i32>,
}

impl Solution {
    /// 有向图
    /// 看是否有环
    /// m[a] = b, a为课程编号，b为依赖a的课程
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if prerequisites.is_empty() {
            return true;
        }

        let mut h = std::collections::HashMap::new();

        for i in prerequisites.iter() {
            h.entry(i[1])
                .or_insert(Node {
                    indegrees: 0,
                    adjacency: vec![],
                })
                .adjacency
                .push(i[0]);

            h.entry(i[0])
                .or_insert(Node {
                    indegrees: 0,
                    adjacency: vec![],
                })
                .indegrees += 1;
        }

        let mut stack = vec![];
        for (&i, node) in h.iter() {
            if node.indegrees == 0 {
                stack.push(i);
            }
        }

        let mut count = 0;

        while let Some(x) = stack.pop() {
            count += 1;

            let node = h.remove(&x).unwrap();

            for i in node.adjacency {
                h.entry(i).and_modify(|x| x.indegrees -= 1);
                if h.get(&i).unwrap().indegrees == 0 {
                    stack.push(i);
                }
            }
        }

        h.is_empty() || count >= num_courses
    }
}
