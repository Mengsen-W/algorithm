/*
 * @Date: 2022-05-15 14:55:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-15 15:54:22
 * @FilePath: /algorithm/812_largest_triangle_area/largest_triangle_area.rs
 */
pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    fn cross(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> i32 {
        (q[0] - p[0]) * (r[1] - q[1]) - (q[1] - p[1]) * (r[0] - q[0])
    }
    fn triangle_area(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> f64 {
        0.5 * (x1 * y2 + x2 * y3 + x3 * y1 - x1 * y3 - x2 * y1 - x3 * y2).abs() as f64
    }
    fn get_convex_hull(points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points = points;
        let n = points.len();
        if n < 4 {
            return points;
        }
        points.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut hull: Vec<Vec<i32>> = Vec::new();

        for p in &points {
            while hull.len() > 1 && cross(&hull[hull.len() - 2], &(hull.last().unwrap()), &p) <= 0 {
                hull.pop();
            }
            hull.push(p.clone());
        }
        let m = hull.len();
        for i in (0..=(n - 2)).rev() {
            while hull.len() > m
                && cross(&hull[hull.len() - 2], &(hull.last().unwrap()), &points[i]) <= 0
            {
                hull.pop();
            }
            hull.push(points[i].to_vec());
        }
        hull.pop();
        hull
    }

    let convex_hull = get_convex_hull(points);
    let n = convex_hull.len();
    let mut ret: f64 = 0.0;
    for i in 0..n {
        let mut j = i + 1;
        let mut k = i + 2;
        while j + 1 < n {
            while k + 1 < n {
                let cur_area = triangle_area(
                    convex_hull[i][0],
                    convex_hull[i][1],
                    convex_hull[j][0],
                    convex_hull[j][1],
                    convex_hull[k][0],
                    convex_hull[k][1],
                );
                let next_area = triangle_area(
                    convex_hull[i][0],
                    convex_hull[i][1],
                    convex_hull[j][0],
                    convex_hull[j][1],
                    convex_hull[k + 1][0],
                    convex_hull[k + 1][1],
                );
                if cur_area >= next_area {
                    break;
                }
                k += 1;
            }
            let area = triangle_area(
                convex_hull[i][0],
                convex_hull[i][1],
                convex_hull[j][0],
                convex_hull[j][1],
                convex_hull[k][0],
                convex_hull[k][1],
            );
            ret = ret.max(area);
            j += 1;
        }
    }
    return ret;
}

fn main() {
    assert_eq!(
        largest_triangle_area(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![0, 2],
            vec![2, 0]
        ]),
        2.0
    );
}
