fn main() {
    let relation = vec![
        vec![0, 2],
        vec![2, 1],
        vec![3, 4],
        vec![2, 3],
        vec![1, 4],
        vec![2, 0],
        vec![0, 4],
    ];
    assert_eq!(3, Solution::num_ways(5, relation, 3));
}

struct Solution;

impl Solution {
    pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut m: HashMap<i32, Vec<i32>> = std::collections::HashMap::with_capacity(n as usize);

        for i in relation.iter() {
            m.entry(i[0])
                .and_modify(|e| e.push(i[1]))
                .or_insert(vec![i[1]]);
        }

        let mut num = 0; // 方案数
        let mut people = vec![0];

        for _ in 0..k {
            let mut p = Vec::new();
            for i in people.into_iter() {
                if let Some(s1) = m.get_mut(&i) {
                    p.append(&mut s1.clone())
                }
            }
            people = p;
        }

        for i in people.iter() {
            if *i == n - 1 {
                num += 1;
            }
        }

        num
    }
}
