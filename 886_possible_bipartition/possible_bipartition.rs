/*
 * @Date: 2022-10-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-16
 * @FilePath: /algorithm/886_possible_bipartition/possible_bipartition.rs
 */

struct Solution;

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut graph: Vec<Vec<i32>> = vec![vec![]; (n + 1) as usize];
        let mut memo: Vec<i32> = vec![0; (n + 1) as usize];
        for pair in dislikes.iter() {
            graph[pair[0] as usize].push(pair[1]);
            graph[pair[1] as usize].push(pair[0]);
        }
        for i in 1..(n + 1) {
            if memo[i as usize] == 0 && !partition(i, &graph, &mut memo, 1) {
                return false;
            }
        }
        true
    }
}
fn partition(i: i32, graph: &Vec<Vec<i32>>, memo: &mut Vec<i32>, group: i32) -> bool {
    memo[i as usize] = group;
    for &neighbor in graph[i as usize].iter() {
        if memo[neighbor as usize] == group {
            return false;
        }
        if memo[neighbor as usize] == 0 && !partition(neighbor, graph, memo, -group) {
            return false;
        }
    }
    true
}

fn main() {
    {
        let n = 4;
        let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 4]];
        assert!(Solution::possible_bipartition(n, dislikes));
    }

    {
        let n = 3;
        let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        assert!(!Solution::possible_bipartition(n, dislikes));
    }

    {
        let n = 5;
        let dislikes = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
        assert!(!Solution::possible_bipartition(n, dislikes));
    }
}
