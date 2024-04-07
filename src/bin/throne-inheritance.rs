#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

struct ThroneInheritance {
    death: std::collections::HashSet<String>,
    tree: std::collections::HashMap<String, Vec<String>>,
    king_name: String,
}

impl ThroneInheritance {

    fn new(kingName: String) -> Self {
        Self {
            death: std::collections::HashSet::new(),
            tree: {
                let mut tree = std::collections::HashMap::new();
                tree.insert(kingName.clone(), vec![]);
                tree
            },
            king_name: kingName,
        }
    }
    
    fn birth(&mut self, parent_name: String, child_name: String) {
        self.tree.entry(parent_name).and_modify(|x|x.push(child_name.clone()));
        self.tree.entry(child_name.clone()).or_insert(vec![]);
    }
    
    fn death(&mut self, name: String) {
        self.death.insert(name);
    }

    fn dfs(&self, name: &String, data: &mut Vec<String>) {
        if !self.death.contains(name) {
            data.push(name.clone());
        }

        if let Some(children) = self.tree.get(name) {
            for child in children {
                self.dfs(child, data);
            }
        }
    }
    
    fn get_inheritance_order(&self) -> Vec<String> {
        let mut data = vec![];

        self.dfs(&self.king_name, &mut data);

        data
    }
}

/**
 * Your ThroneInheritance object will be instantiated and called as such:
 * let obj = ThroneInheritance::new(kingName);
 * obj.birth(parentName, childName);
 * obj.death(name);
 * let ret_3: Vec<String> = obj.get_inheritance_order();
 */
