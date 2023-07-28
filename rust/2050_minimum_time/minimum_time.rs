/*
 * @Date: 2023-07-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-28
 * @FilePath: /algorithm/rust/2050_minimum_time/minimum_time.rs
 */

struct Solution;

impl Solution {
    fn build_graph_by_relations(relations: &Vec<Vec<i32>>, n: usize) -> (Vec<Vec<i32>>, Vec<i32>) {
        let mut ret = vec![vec![]; n];
        let mut into = vec![0; n];
        for relation in relations.iter() {
            ret[relation[0] as usize - 1].push(relation[1] - 1);
            into[relation[1] as usize - 1] += 1;
        }
        (ret, into)
    }

    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        use std::collections::VecDeque;
        let n = n as usize;
        let (grph, mut into) = Self::build_graph_by_relations(&relations, n);
        let mut f = vec![0; n];
        let mut q: VecDeque<i32> = into
            .iter()
            .enumerate()
            .filter(|(_i, &x)| x == 0)
            .map(|(i, &_x)| i as i32)
            .collect();

        while !q.is_empty() {
            let x = *q.front().unwrap() as usize;
            q.pop_front();
            for &y in grph[x].iter() {
                let y = y as usize;
                if f[x] + time[x] > f[y] {
                    f[y] = f[x] + time[x];
                }
                into[y] -= 1;
                if into[y] == 0 {
                    q.push_back(y as i32);
                }
            }
        }

        f.iter().zip(time.iter()).map(|(x, y)| x + y).max().unwrap()
    }
}

fn main() {
    let tests = vec![
        (3, vec![vec![1, 3], vec![2, 3]], vec![2, 3, 5], 8),
        (
            5,
            vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]],
            vec![1, 2, 3, 4, 5],
            12,
        ),
    ];

    for (n, relations, time, expect) in tests {
        assert_eq!(Solution::minimum_time(n, relations, time), expect);
    }
}
