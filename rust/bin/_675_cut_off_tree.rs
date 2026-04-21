/*
 * @Date: 2022-05-23 18:57:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-23 19:00:32
 * @FilePath: /algorithm/675_cut_off_tree/cut_off_tree.rs
 */

pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
    fn a_star(map: &Vec<Vec<i32>>, start: (i32, i32), end: (i32, i32)) -> i32 {
        use std::cmp::Reverse;
        fn ham(p1: (i32, i32), p2: (i32, i32)) -> i32 {
            (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
        }
        let (n, m) = (map.len() as i32, map[0].len() as i32);
        let tab = [-1, 0, 1, 0, -1];
        let mut used = vec![vec![false; m as usize]; n as usize];
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(Reverse((ham(start, end), 0, start)));
        used[start.0 as usize][start.1 as usize] = true;
        while let Some(Reverse((_, step, p))) = heap.pop() {
            if p == end {
                return step;
            }
            for i in 0..4 {
                let (x, y) = (p.0 + tab[i], p.1 + tab[i + 1]);
                if x < 0 || x >= n || y < 0 || y >= m {
                    continue;
                }
                if map[x as usize][y as usize] == 0 {
                    continue;
                }
                if used[x as usize][y as usize] {
                    continue;
                }
                let cost = ham((x, y), end) + step + 1;
                heap.push(Reverse((cost, step + 1, (x, y))));
                used[x as usize][y as usize] = true;
            }
        }
        -1
    }
    let (n, m) = (forest.len(), forest[0].len());
    let mut points = Vec::with_capacity(n * m);
    forest.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &t)| {
            if t > 1 {
                points.push((t, i as i32, j as i32))
            }
        })
    });
    points.push((0, 0, 0));
    points.sort_unstable();
    let mut ans = 0;
    for i in 0..points.len() - 1 {
        let p1 = (points[i].1, points[i].2);
        let p2 = (points[i + 1].1, points[i + 1].2);
        let d = a_star(&forest, p1, p2);
        if d < 0 {
            return -1;
        }
        ans += d;
    }
    ans
}

fn main() {
    assert_eq!(
        cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]]),
        6
    );
    assert_eq!(
        cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 0], vec![7, 6, 5]]),
        -1
    );
    assert_eq!(
        cut_off_tree(vec![vec![2, 3, 4], vec![0, 0, 5], vec![8, 7, 6]]),
        6
    );
}
