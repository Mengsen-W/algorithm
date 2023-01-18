/*
 * @Date: 2021-08-22 13:17:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-22 13:44:55
 */

struct Solution;

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let source = vec![0, 0];
        let manhattan_distance = |point1: &Vec<i32>, point2: &Vec<i32>| {
            (point1[0] - point2[0]).abs() + (point1[1] - point2[1]).abs()
        };
        let distance = manhattan_distance(&source, &target);
        for ghost in &ghosts {
            let ghost_distance = manhattan_distance(ghost, &target);
            if ghost_distance <= distance {
                return false;
            }
        }
        true
    }
}

fn main() {
    {
        let ghosts = vec![vec![1, 0], vec![0, 3]];
        let target = vec![0, 1];
        assert!(Solution::escape_ghosts(ghosts, target));
    }
    {
        let ghosts = vec![vec![1, 0]];
        let target = vec![2, 0];
        assert!(!Solution::escape_ghosts(ghosts, target));
    }
    {
        let ghosts = vec![vec![2, 0]];
        let target = vec![1, 0];
        assert!(!Solution::escape_ghosts(ghosts, target));
    }
    {
        let ghosts = vec![
            vec![5, 0],
            vec![-10, -2],
            vec![0, -5],
            vec![-2, -2],
            vec![-7, 1],
        ];
        let target = vec![7, 7];
        assert!(!Solution::escape_ghosts(ghosts, target));
    }
    {
        let ghosts = vec![
            vec![-1, 0],
            vec![0, 1],
            vec![-1, 0],
            vec![0, 1],
            vec![-1, 0],
        ];
        let target = vec![0, 0];
        assert!(Solution::escape_ghosts(ghosts, target));
    }
}
