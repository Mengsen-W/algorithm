/*
 * @Date: 2022-02-18 02:02:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-18 04:10:00
 */

pub fn find_center1(edges: Vec<Vec<i32>>) -> i32 {
    let n = edges.len();
    let degrees: Vec<i32> = edges.iter().fold(vec![0; n + 2], |mut degrees, edge| {
        degrees[edge[0] as usize] += 1;
        degrees[edge[1] as usize] += 1;
        degrees
    });
    for (k, v) in degrees.iter().enumerate() {
        if *v == n as i32 {
            return k as i32;
        }
    }
    0
}

pub fn find_center2(edges: Vec<Vec<i32>>) -> i32 {
    if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
        edges[0][0]
    } else {
        edges[0][1]
    }
}

fn main() {
    assert_eq!(find_center1(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
    assert_eq!(find_center2(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
    assert_eq!(
        find_center1(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
        1
    );
    assert_eq!(
        find_center2(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
        1
    );
}
