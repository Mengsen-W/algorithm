/*
 * @Date: 2023-10-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-31
 * @FilePath: /algorithm/rust/2003_smallest_missing_value_subtree/smallest_missing_value_subtree.rs
 */
struct Solution;

impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        use std::collections::{HashMap, HashSet};
        let n = parents.len();
        let record = {
            let mut record = HashMap::new();
            for i in 0..n {
                record.insert(nums[i], i);
            }
            record
        };
        let mut ret = vec![1; n];
        let mut path = HashSet::new();
        let mut tail;
        if let Some(&(mut node)) = record.get(&1) {
            tail = node;
            loop {
                path.insert(node);
                if node == 0 {
                    break;
                }
                node = parents[node] as usize;
            }
        } else {
            return ret;
        }
        'L: for val in 2.. {
            let mut anc = if let Some(&node) = record.get(&val) {
                node
            } else {
                for node in path {
                    ret[node] = val;
                }
                break;
            };
            while !path.contains(&anc) {
                if nums[anc] < val {
                    continue 'L;
                }
                anc = parents[anc] as usize;
            }
            while tail != anc {
                ret[tail] = val;
                path.remove(&tail);
                tail = parents[tail] as usize;
            }
        }
        ret
    }
}

fn main() {
    let tests = vec![
        (vec![-1, 0, 0, 2], vec![1, 2, 3, 4], vec![5, 1, 1, 1]),
        (
            vec![-1, 0, 1, 0, 3, 3],
            vec![5, 4, 6, 2, 1, 3],
            vec![7, 1, 1, 4, 2, 1],
        ),
        (
            vec![-1, 2, 3, 0, 2, 4, 1],
            vec![2, 3, 4, 5, 6, 7, 8],
            vec![1, 1, 1, 1, 1, 1, 1],
        ),
    ];

    for (parents, nums, ans) in tests {
        assert_eq!(Solution::smallest_missing_value_subtree(parents, nums), ans);
    }
}
