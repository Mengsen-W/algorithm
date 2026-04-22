/*
 * @Date: 2024-04-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-13
 * @FilePath: /algorithm/rust/2924_find_champion/find_champion.rs
 */

struct Solution;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut degree = vec![0; n as usize];
        for e in edges.iter() {
            degree[e[1] as usize] += 1;
        }
        let mut champion = -1;
        for i in 0..n {
            if degree[i as usize] == 0 {
                if champion == -1 {
                    champion = i;
                } else {
                    return -1;
                }
            }
        }
        champion
    }
}

fn main() {
    let tests = vec![
        (3, vec![vec![0, 1], vec![1, 2]], 0),
        (4, vec![vec![0, 2], vec![1, 3], vec![1, 2]], -1),
    ];

    for (n, edges, ans) in tests {
        assert_eq!(Solution::find_champion(n, edges), ans);
    }
}
