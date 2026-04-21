/*
 * @Date: 2023-11-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-09
 * @FilePath: /algorithm/rust/2258_maximum_minutes/maximum_minutes.rs
 */

struct Solution;

use std::cell::RefCell;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let mut left = 0;
        let mut right = grid.len() * grid[0].len() + 1;

        if !Self::check(left, &grid) {
            return -1i32;
        }

        while left + 1 < right {
            let mid = (left + right) >> 1;
            if Self::check(mid, &grid) {
                left = mid;
            } else {
                right = mid;
            }
        }

        if left < grid.len() * grid[0].len() {
            left as i32
        } else {
            10i32.pow(9)
        }
    }

    pub fn check(t: usize, grid: &Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut fires = {
            let mut rt: Vec<(usize, usize)> = vec![];
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        rt.push((i, j));
                    }
                }
            }
            rt
        };
        let fire_set: HashSet<(usize, usize)> = HashSet::from_iter(fires.clone().into_iter());
        let fire_set = Rc::new(RefCell::new(fire_set));

        let mut spread_fire = || {
            let tmp = &fires;
            let mut new_fires: Vec<(usize, usize)> = vec![];
            for &(i, j) in tmp.iter() {
                let targets = {
                    let mut rt: Vec<(usize, usize)> = vec![];
                    if i >= 1 {
                        rt.push((i - 1, j));
                    }
                    if i < m - 1 {
                        rt.push((i + 1, j));
                    }
                    if j >= 1 {
                        rt.push((i, j - 1));
                    }
                    if j < n - 1 {
                        rt.push((i, j + 1));
                    }
                    rt
                };
                let mut fire_set_mut = fire_set.borrow_mut();
                for &(x, y) in targets.iter() {
                    if (grid[x][y] == 0) && !fire_set_mut.contains(&(x, y)) {
                        fire_set_mut.insert((x, y));
                        new_fires.push((x, y));
                    }
                }
            }
            fires = new_fires;
        };
        for _ in 0..t {
            spread_fire();
        }
        if fire_set.borrow().contains(&(0, 0)) {
            return false;
        }

        let mut people_can_go: Vec<(usize, usize)> = vec![(0, 0)];
        let mut visited: HashSet<(usize, usize)> =
            HashSet::from_iter(people_can_go.clone().into_iter());

        while people_can_go.len() > 0 {
            let tmp = &people_can_go;
            let mut new_people_can_go: Vec<(usize, usize)> = vec![];
            for &(i, j) in tmp.iter() {
                if fire_set.borrow().contains(&(i, j)) {
                    continue;
                }
                // there can go and find more
                let targets = {
                    let mut rt: Vec<(usize, usize)> = vec![];
                    if i >= 1 {
                        rt.push((i - 1, j));
                    }
                    if i < m - 1 {
                        rt.push((i + 1, j));
                    }
                    if j >= 1 {
                        rt.push((i, j - 1));
                    }
                    if j < n - 1 {
                        rt.push((i, j + 1));
                    }
                    rt
                };
                for &(x, y) in targets.iter() {
                    if (grid[x][y] == 0)
                        && !fire_set.borrow().contains(&(x, y))
                        && !visited.contains(&(x, y))
                    {
                        if x == m - 1 && y == n - 1 {
                            return true;
                        }

                        new_people_can_go.push((x, y));
                        visited.insert((x, y));
                    }
                }
            }
            people_can_go = new_people_can_go;
            spread_fire();
        }

        false
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![0, 2, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 2, 2, 1, 0],
                vec![0, 2, 0, 0, 1, 2, 0],
                vec![0, 0, 2, 2, 2, 0, 2],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
            3,
        ),
        (
            vec![vec![0, 0, 0, 0], vec![0, 1, 2, 0], vec![0, 2, 0, 0]],
            -1,
        ),
        (
            vec![vec![0, 0, 0], vec![2, 2, 0], vec![1, 2, 0]],
            1000000000,
        ),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::maximum_minutes(grid), ans);
    }
}
