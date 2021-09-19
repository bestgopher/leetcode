fn main() {}

struct Solution;

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
struct RandomizedSet {
    set: std::cell::RefCell<std::collections::HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            set: std::cell::RefCell::new(std::collections::HashSet::default()),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&self, val: i32) -> bool {
        self.set.borrow_mut().insert(val)
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&self, val: i32) -> bool {
        self.set.borrow_mut().remove(&val)
    }

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let s = rng.gen_range(0..self.set.borrow().len());
        for (i, v) in self.set.borrow().iter().enumerate() {
            if i == s {
                return *v;
            }
        }

        0
    }
}
