#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

struct StockPrice {
    current: i32,

    /// timestamp: price
    time_map: std::collections::HashMap<i32, i32>,
    /// price: 次数
    price_count: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */

impl StockPrice {
    fn new() -> Self {
        StockPrice {
            current: 0,
            time_map: Default::default(),
            price_count: Default::default(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.current = self.current.max(timestamp);
        if let Some(x) = self.time_map.insert(timestamp, price) {
            *self.price_count.get_mut(&x).unwrap() -= 1;
            if self.price_count[&x] == 0 {
                self.price_count.remove(&x);
            }
        }

        self.price_count
            .entry(price)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    fn current(&self) -> i32 {
        self.time_map[&self.current]
    }

    fn maximum(&self) -> i32 {
        *self.price_count.iter().rev().next().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.price_count.iter().next().unwrap().0
    }
}
