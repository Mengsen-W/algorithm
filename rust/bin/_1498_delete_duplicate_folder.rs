struct Solution;

use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    serial: String,                  // 当前节点结构的序列化表示
    children: HashMap<String, Trie>, // 当前节点的子节点
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut root = Trie::default(); // 根节点
                                        // 构建字典树
        for path in paths {
            let mut cur = &mut root;
            for node in path {
                cur = cur.children.entry(node.clone()).or_default();
            }
        }

        let mut freq = HashMap::new(); // 哈希表记录每一种序列化表示的出现次数

        // 基于深度优先搜索的后序遍历，计算每一个节点结构的序列化表示
        fn construct(node: &mut Trie, freq: &mut HashMap<String, usize>) {
            if node.children.is_empty() {
                return; // 如果是叶节点，无需操作
            }

            let mut v = Vec::new();
            for (folder, child) in node.children.iter_mut() {
                construct(child, freq);
                v.push(format!("{}({})", folder, child.serial));
            }

            v.sort();
            node.serial = v.join("");
            *freq.entry(node.serial.clone()).or_default() += 1;
        }
        construct(&mut root, &mut freq);
        let mut ans = Vec::new();
        let mut path = Vec::new();

        // 操作字典树，删除重复文件夹
        fn operate(
            node: &Trie,
            freq: &HashMap<String, usize>,
            path: &mut Vec<String>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if freq.get(&node.serial).unwrap_or(&0) > &1 {
                return; // 如果序列化表示出现超过1次，需要删除
            }

            if !path.is_empty() {
                ans.push(path.clone());
            }

            for (folder, child) in &node.children {
                path.push(folder.clone());
                operate(child, freq, path, ans);
                path.pop();
            }
        }
        operate(&root, &freq, &mut path, &mut ans);

        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec!["a"],
                vec!["c"],
                vec!["d"],
                vec!["a", "b"],
                vec!["c", "b"],
                vec!["d", "a"],
            ],
            vec![vec!["d"], vec!["d", "a"]],
        ),
        (
            vec![
                vec!["a"],
                vec!["c"],
                vec!["a", "b"],
                vec!["c", "b"],
                vec!["a", "b", "x"],
                vec!["a", "b", "x", "y"],
                vec!["w"],
                vec!["w", "y"],
            ],
            vec![vec!["c"], vec!["c", "b"], vec!["a"], vec!["a", "b"]],
        ),
        (
            vec![vec!["a", "b"], vec!["c", "d"], vec!["c"], vec!["a"]],
            vec![vec!["c"], vec!["c", "d"], vec!["a"], vec!["a", "b"]],
        ),
        (
            vec![
                vec!["a"],
                vec!["a", "x"],
                vec!["a", "x", "y"],
                vec!["a", "z"],
                vec!["b"],
                vec!["b", "x"],
                vec!["b", "x", "y"],
                vec!["b", "z"],
            ],
            vec![],
        ),
        (
            vec![
                vec!["a"],
                vec!["a", "x"],
                vec!["a", "x", "y"],
                vec!["a", "z"],
                vec!["b"],
                vec!["b", "x"],
                vec!["b", "x", "y"],
                vec!["b", "z"],
                vec!["b", "w"],
            ],
            vec![
                vec!["b"],
                vec!["b", "w"],
                vec!["b", "z"],
                vec!["a"],
                vec!["a", "z"],
            ],
        ),
    ];

    for (paths, ans) in tests {
        assert_eq!(
            Solution::delete_duplicate_folder(
                paths
                    .iter()
                    .map(|v| v.iter().map(|s| s.to_string()).collect::<Vec<String>>())
                    .collect()
            ),
            ans.iter()
                .map(|v| v.iter().map(|s| s.to_string()).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>()
        )
    }
}
