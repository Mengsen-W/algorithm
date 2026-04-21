/*
 * @Date: 2023-01-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-24
 * @FilePath: /algorithm/rust/1828_count_points/count_points.rs
 */

pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    queries
        .iter()
        .map(|circle| {
            let (cx, cy, cr) = (
                circle.get(0).unwrap(),
                circle.get(1).unwrap(),
                circle.get(2).unwrap(),
            );
            points
                .iter()
                .filter(|&point| {
                    let (px, py) = (point.get(0).unwrap(), point.get(1).unwrap());
                    (px - cx) * (px - cx) + (py - cy) * (py - cy) <= cr * cr
                })
                .count() as i32
        })
        .collect()
}

fn main() {
    {
        let points = [[1, 3], [3, 3], [5, 3], [2, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let queries = [[2, 3, 1], [4, 3, 1], [1, 1, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = [3, 2, 2].to_vec();
        assert_eq!(count_points(points, queries), ans);
    }

    {
        let points = [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let queries = [[1, 2, 2], [2, 2, 2], [4, 3, 2], [4, 3, 3]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = [2, 3, 2, 4].to_vec();
        assert_eq!(count_points(points, queries), ans);
    }
}
