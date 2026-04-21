/*
 * @Date: 2022-02-16 03:04:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-16 09:16:46
 */

pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
    use std::collections::{HashMap, HashSet};
    let adj: HashMap<i32, HashSet<i32>> = pairs.iter().fold(HashMap::new(), |mut map, pair| {
        map.entry(pair[0]).or_insert(HashSet::new()).insert(pair[1]);
        map.entry(pair[1]).or_insert(HashSet::new()).insert(pair[0]);
        map
    });

    let mut root = -1;
    for (node, neighbours) in &adj {
        if neighbours.len() == adj.len() - 1 {
            root = *node;
            break;
        }
    }

    if root == -1 {
        return 0;
    }
    let mut res = 1;
    for (node, neighbours) in &adj {
        if *node == root {
            continue;
        }
        let curr_degree = neighbours.len();
        let mut parent = 1;
        let mut parent_degree = usize::MAX;
        for neighbour in neighbours {
            if adj.get(neighbour).unwrap_or(&HashSet::new()).len() < parent_degree
                && adj.get(neighbour).unwrap_or(&HashSet::new()).len() >= curr_degree
            {
                parent = *neighbour;
                parent_degree = adj.get(neighbour).unwrap_or(&HashSet::new()).len();
            }
        }
        if parent == -1 {
            return 0;
        }
        for neighbour in neighbours {
            if *neighbour == parent {
                continue;
            }
            if !adj
                .get(&parent)
                .unwrap_or(&HashSet::new())
                .contains(neighbour)
            {
                return 0;
            }
        }
        if parent_degree == curr_degree {
            res = 2;
        }
    }
    res
}

fn main() {
    assert_eq!(check_ways(vec![vec![1, 2], vec![2, 3]]), 1);
    assert_eq!(check_ways(vec![vec![1, 2], vec![2, 3], vec![1, 3]]), 2);
    assert_eq!(
        check_ways(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![1, 5]]),
        0
    );

    assert_eq!(check_ways(vec![vec![1, 2]]), 2);
}
