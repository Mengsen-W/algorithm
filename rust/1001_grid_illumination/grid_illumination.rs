/*
 * @Date: 2022-02-08 00:28:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-08 00:51:39
 */

pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};
    let (mut points, mut row, mut col, mut diag, mut anti_diag) = (
        HashSet::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    );

    for lamp in lamps {
        let (r, c) = (lamp[0], lamp[1]);
        if points.contains(&(r, c)) {
            continue;
        }
        points.insert((r, c));
        *row.entry(r).or_insert(0) += 1;
        *col.entry(c).or_insert(0) += 1;
        *diag.entry(r - c).or_insert(0) += 1;
        *anti_diag.entry(r + c).or_insert(0) += 1;
    }

    let is_valid = |mp: &HashMap<i32, i32>, k: &i32| -> bool {
        if let Some(v) = mp.get(k) {
            *v > 0
        } else {
            false
        }
    };

    let mut ans = Vec::new();
    for query in queries.iter() {
        let (r, c) = (query[0], query[1]);
        ans.push(
            (is_valid(&row, &r)
                || is_valid(&col, &c)
                || is_valid(&diag, &(r - c))
                || is_valid(&anti_diag, &(r + c))) as i32,
        );
        for x in r - 1..=r + 1 {
            for y in c - 1..=c + 1 {
                if x < 0 || x >= n || y < 0 || y >= n || !points.contains(&(x, y)) {
                    continue;
                }
                points.remove(&(x, y));
                *row.get_mut(&x).unwrap() -= 1;
                *col.get_mut(&y).unwrap() -= 1;
                *diag.get_mut(&(x - y)).unwrap() -= 1;
                *anti_diag.get_mut(&(x + y)).unwrap() -= 1;
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(
        grid_illumination(
            5,
            vec![vec![0, 0], vec![4, 4]],
            vec![vec![1, 1], vec![1, 0]]
        ),
        vec![1, 0]
    );

    assert_eq!(
        grid_illumination(
            5,
            vec![vec![0, 0], vec![4, 4]],
            vec![vec![1, 1], vec![1, 1]]
        ),
        vec![1, 1]
    );

    assert_eq!(
        grid_illumination(
            5,
            vec![vec![0, 0], vec![0, 4]],
            vec![vec![0, 4], vec![0, 1], vec![1, 4]]
        ),
        vec![1, 1, 0]
    );
}
