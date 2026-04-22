/*
 * @Date: 2022-07-18
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-18
 * @FilePath: /algorithm/749_contain_virus/contain_virus.rs
 */

use std::collections::{BinaryHeap, VecDeque};

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct InfectedAreaInfo {
    next_infect_cnt: i32,
    needed_wall_cnt: i32,
    pos: (usize, usize),
}

struct Solution;

impl Solution {
    pub fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        let mut area = is_infected;
        let (m, n) = (area.len(), area[0].len());
        let mut result = 0;
        loop {
            let mut infos = BinaryHeap::new();
            (0..m).for_each(|i| {
                (0..n).for_each(|j| {
                    if area[i][j] == 1 {
                        let ret = Self::mark(&mut area, (i, j));
                        Self::remove_edge_marks(&mut area);
                        infos.push(InfectedAreaInfo {
                            pos: (i, j),
                            needed_wall_cnt: ret.0,
                            next_infect_cnt: ret.1,
                        });
                    }
                })
            });
            Self::remove_virus_marks(&mut area);
            if infos.is_empty() {
                break;
            }
            let most_dangerous = infos.pop().unwrap();
            Self::contain(&mut area, most_dangerous.pos);
            result += most_dangerous.needed_wall_cnt;
            Self::virus_spread(&mut area);
        }
        result
    }

    fn mark(area: &mut Vec<Vec<i32>>, pos: (usize, usize)) -> (i32, i32) {
        Self::bfs_virus(area, pos, -1, -2)
    }

    fn contain(area: &mut Vec<Vec<i32>>, pos: (usize, usize)) {
        Self::bfs_virus(area, pos, 2, 0);
    }

    fn virus_spread(area: &mut Vec<Vec<i32>>) {
        (0..area.len()).for_each(|i| {
            (0..area[0].len()).for_each(|j| {
                if area[i][j] == 1 {
                    Self::bfs_virus(area, (i, j), -1, -3);
                }
            })
        });
        Self::remove_virus_marks(area);
    }

    fn remove_edge_marks(area: &mut Vec<Vec<i32>>) {
        (0..area.len()).for_each(|i| {
            (0..area[0].len()).for_each(|j| {
                if area[i][j] == -2 {
                    area[i][j] = 0
                }
            })
        });
    }

    fn remove_virus_marks(area: &mut Vec<Vec<i32>>) {
        (0..area.len()).for_each(|i| {
            (0..area[0].len()).for_each(|j| {
                if area[i][j] == -1 || area[i][j] == -3 {
                    area[i][j] = 1;
                }
            })
        });
    }

    fn bfs_virus(
        area: &mut Vec<Vec<i32>>,
        pos: (usize, usize),
        virus_mark: i32,
        edge_mark: i32,
    ) -> (i32, i32) {
        let (m, n) = (area.len(), area[0].len());
        let mut next_infect_cnt = 0;
        let mut needed_wall_cnt = 0;

        let mut queue = VecDeque::new();
        queue.push_back(pos);
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            if x >= m || y >= n {
                continue;
            }
            match area[x][y] {
                0 => {
                    area[x][y] = edge_mark;
                    next_infect_cnt += 1;
                    needed_wall_cnt += 1;
                }
                1 => {
                    area[x][y] = virus_mark;
                    queue.push_back((x + 1, y));
                    queue.push_back((x - 1, y));
                    queue.push_back((x, y + 1));
                    queue.push_back((x, y - 1));
                }
                n if n == edge_mark => needed_wall_cnt += 1,
                _ => (),
            }
        }
        (needed_wall_cnt, next_infect_cnt)
    }
}

fn main() {
    assert_eq!(
        Solution::contain_virus(vec![
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0]
        ]),
        10
    );
    assert_eq!(
        Solution::contain_virus(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        4
    );
    assert_eq!(
        Solution::contain_virus(vec![
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0]
        ]),
        13
    );
}
