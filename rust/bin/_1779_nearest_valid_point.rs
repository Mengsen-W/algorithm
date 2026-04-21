/*
 * @Date: 2022-12-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-01
 * @FilePath: /algorithm/1779_nearest_valid_point/nearest_valid_point.rs
 */

pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    let (mut min_idx, mut min_distance) = (-1, i32::MAX);
    for i in 0..points.len() {
        if x == points[i][0] || y == points[i][1] {
            let distance = (x - points[i][0]).abs() + (y - points[i][1]).abs();
            if distance < min_distance {
                min_distance = distance;
                min_idx = i as i32;
            }
        }
    }
    min_idx
}

fn main() {
    {
        let x = 3;
        let y = 4;
        let points = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
        let ans = 2;
        assert_eq!(nearest_valid_point(x, y, points), ans);
    }

    {
        let x = 3;
        let y = 4;
        let points = vec![vec![3, 4]];
        let ans = 0;
        assert_eq!(nearest_valid_point(x, y, points), ans);
    }
}
