struct Solution;

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut points = vec![0i64; n];
        for (i, &edge) in edges.iter().enumerate() {
            points[edge as usize] += i as i64;
        }
        let mut max_points = -1i64;
        let mut res = -1i32;
        for (i, &points_val) in points.iter().enumerate() {
            if points_val > max_points {
                max_points = points_val;
                res = i as i32;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![1, 0, 0, 0, 0, 7, 7, 5], 7), (vec![2, 0, 0, 2], 0)];

    for (edges, ans) in tests {
        assert_eq!(Solution::edge_score(edges), ans);
    }
}
