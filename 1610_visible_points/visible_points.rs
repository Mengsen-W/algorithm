/*
 * @Date: 2021-12-17 07:51:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-17 08:25:17
 */

pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
    let mut same_count = 0;
    let mut polar_degree: Vec<f64> = Vec::new();
    for point in points {
        if point[0] == location[0] && point[1] == location[1] {
            same_count += 1;
            continue;
        }
        let degree: f64 = ((point[1] - location[1]) as f64).atan2((point[0] - location[0]).into());
        polar_degree.push(degree);
    }
    polar_degree.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let m = polar_degree.len();
    (0..m).for_each(|i| polar_degree.push(polar_degree[i] + 2.0 * std::f64::consts::PI));
    let mut max_count = 0;
    let mut right = 0;
    let degree: f64 = angle as f64 * std::f64::consts::PI / 180.0;
    (0..m).for_each(|i| {
        let polar_degree_size = polar_degree.len();
        while (right < polar_degree_size) && (polar_degree[right] <= polar_degree[i] + degree) {
            right += 1;
        }
        max_count = max_count.max(right - i);
    });

    (max_count + same_count) as i32
}

fn main() {
    assert_eq!(
        visible_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]], 90, vec![1, 1]),
        3
    );

    assert_eq!(
        visible_points(
            vec![vec![2, 1], vec![2, 2], vec![3, 4], vec![1, 1]],
            90,
            vec![1, 1]
        ),
        4
    );

    assert_eq!(
        visible_points(vec![vec![1, 0], vec![2, 1]], 13, vec![1, 1]),
        1
    );
}
