/*
 * @Date: 2021-08-14 13:51:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-14 14:21:41
 */

struct Solution;

impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut order = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n - 1 {
                order[i][preferences[i][j] as usize] = j;
            }
        }
        let mut mat = vec![0; n];
        for pr in &pairs {
            mat[pr[0] as usize] = pr[1];
            mat[pr[1] as usize] = pr[0];
        }
        let mut un_happy_count = 0;
        for x in 0..n {
            let index = order[x][mat[x] as usize];
            for i in 0..index {
                let u = preferences[x][i] as usize;
                let v = mat[u] as usize;
                if order[u][x] < order[u][v] {
                    un_happy_count += 1;
                    break;
                }
            }
        }
        un_happy_count
    }
}

fn main() {
    {
        let n = 4;
        let preferences = vec![vec![1, 2, 3], vec![3, 2, 0], vec![3, 1, 0], vec![1, 2, 0]];
        let pairs = vec![vec![0, 1], vec![2, 3]];
        let ans = 2;
        assert_eq!(Solution::unhappy_friends(n, preferences, pairs), ans);
    }
    {
        let n = 2;
        let preferences = vec![vec![1], vec![0]];
        let pairs = vec![vec![1, 0]];
        let ans = 0;
        assert_eq!(Solution::unhappy_friends(n, preferences, pairs), ans);
    }
    {
        let n = 4;
        let preferences = vec![vec![1, 3, 2], vec![2, 3, 0], vec![1, 3, 0], vec![0, 2, 1]];
        let pairs = vec![vec![1, 3], vec![0, 2]];
        let ans = 4;
        assert_eq!(Solution::unhappy_friends(n, preferences, pairs), ans);
    }
}
