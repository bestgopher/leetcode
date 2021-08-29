fn main() {}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
struct Solution {
    index_map: std::collections::HashMap<i32, Vec<usize>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut index_map = std::collections::HashMap::new();

        nums
            .into_iter()
            .enumerate()
            .for_each(|(index, value)| {
                index_map.entry(value)
                    .and_modify(|y: &mut Vec<usize>| y.push(index)).or_insert(vec![index]);
            });

        Self { index_map }
    }

    fn pick(&self, target: i32) -> i32 {
        use rand::Rng;

        match self.index_map.get(&target) {
            Some(x) => {
                x[rand::thread_rng().gen_range(0..x.len())] as i32
            }
            None => 0
        }
    }
}
