struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut x0 = points[0][0];
        let mut y0 = points[0][1];
        let mut ans = 0;

        for i in 1..points.len() {
            let x1 = points[i][0];
            let y1 = points[i][1];
            ans += (x0 - x1).abs().max((y0 - y1).abs());
            x0 = x1;
            y0 = y1;
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 1], vec![3, 4], vec![-1, 0]], 7),
        (vec![vec![3, 2], vec![-2, 2]], 5),
    ];

    for (points, expected) in tests {
        assert_eq!(Solution::min_time_to_visit_all_points(points), expected);
    }
}
