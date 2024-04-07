/*
 * @Date: 2024-04-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-07
 * @FilePath: /algorithm/rust/1600_ThroneInheritance/ThroneInheritance.rs
 */

use std::collections::{HashMap, HashSet};

struct ThroneInheritance {
    persons: HashMap<String, Vec<String>>,
    dead: HashSet<String>,
    king: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ThroneInheritance {
    fn new(king_name: String) -> Self {
        let mut res = ThroneInheritance {
            persons: HashMap::new(),
            dead: HashSet::new(),
            king: king_name.clone(),
        };
        res.persons.insert(king_name, Vec::new());
        res
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        self.persons.insert(child_name.clone(), Vec::new());
        self.persons.get_mut(&parent_name).unwrap().push(child_name);
    }

    fn death(&mut self, name: String) {
        self.dead.insert(name);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        let mut res = Vec::new();
        self.preorder(&self.king, &mut res);
        return res;
    }

    fn preorder(&self, name: &String, res: &mut Vec<String>) {
        if !self.dead.contains(name) {
            res.push(name.clone());
        }
        if self.persons.contains_key(name) {
            for child in &self.persons[name] {
                self.preorder(child, res);
            }
        }
    }
}

fn main() {
    let mut t = ThroneInheritance::new("king".to_string()); // 继承顺序：king
    t.birth("king".to_string(), "andy".to_string()); // 继承顺序：king > andy
    t.birth("king".to_string(), "bob".to_string()); // 继承顺序：king > andy > bob
    t.birth("king".to_string(), "catherine".to_string()); // 继承顺序：king > andy > bob > catherine
    t.birth("andy".to_string(), "matthew".to_string()); // 继承顺序：king > andy > matthew > bob > catherine
    t.birth("bob".to_string(), "alex".to_string()); // 继承顺序：king > andy > matthew > bob > alex > catherine
    t.birth("bob".to_string(), "asha".to_string()); // 继承顺序：king > andy > matthew > bob > alex > asha > catherine
    assert_eq!(
        t.get_inheritance_order(),
        vec![
            "king",
            "andy",
            "matthew",
            "bob",
            "alex",
            "asha",
            "catherine"
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    ); // 返回 ["king", "andy", "matthew", "bob", "alex", "asha", "catherine"]
    t.death("bob".to_string()); // 继承顺序：king > andy > matthew > bob（已经去世）> alex > asha > catherine
    assert_eq!(
        t.get_inheritance_order(),
        vec!["king", "andy", "matthew", "alex", "asha", "catherine"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    ); // 返回 ["king", "andy", "matthew", "alex", "asha", "catherine"]
}
