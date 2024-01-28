#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        let mut set = std::collections::HashSet::<(i32, i32)>::new();

        Self::dfs(
            jug1_capacity,
            jug2_capacity,
            target_capacity,
            0,
            0,
            &mut set,
        )
    }

    fn dfs(
        jug1_capacity: i32,
        jug2_capacity: i32,
        target_capacity: i32,
        current_1: i32,
        current_2: i32,
        set: &mut std::collections::HashSet<(i32, i32)>,
    ) -> bool {
        if set.contains(&(current_1, current_2)) {
            return false;
        }
        set.insert((current_1, current_2));

        if current_1 == target_capacity
            || current_2 == target_capacity
            || current_1 + current_2 == target_capacity
        {
            return true;
        }

        // 1清空
        // 2清空
        // 1装满
        // 2装满
        // 1到给2
        // 2到给1
        Self::dfs(
            jug1_capacity,
            jug2_capacity,
            target_capacity,
            0,
            current_2,
            set,
        ) || Self::dfs(
            jug1_capacity,
            jug2_capacity,
            target_capacity,
            current_1,
            0,
            set,
        ) || Self::dfs(
            jug1_capacity,
            jug2_capacity,
            target_capacity,
            jug1_capacity,
            current_2,
            set,
        ) || Self::dfs(
            jug1_capacity,
            jug2_capacity,
            target_capacity,
            current_1,
            jug2_capacity,
            set,
        ) || Self::dfs(
            jug1_capacity,
            jug2_capacity,
            target_capacity,
            current_1 - (jug2_capacity - current_2).max(0).min(current_1),
            current_2 + (jug2_capacity - current_2).max(0).min(current_1),
            set,
        ) || Self::dfs(
            jug1_capacity,
            jug2_capacity,
            target_capacity,
            current_1 + (jug1_capacity - current_1).max(0).min(jug2_capacity),
            jug2_capacity - (jug1_capacity - current_1).max(0).min(jug2_capacity),
            set,
        )
    }
}
