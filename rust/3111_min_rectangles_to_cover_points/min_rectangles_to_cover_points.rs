struct Solution;

impl Solution {
    pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        let mut points = points.clone();
        points.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut res = 0;
        let mut bound = -1;
        for p in points {
            if p[0] > bound {
                bound = p[0] + w;
                res += 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![2, 1],
                vec![1, 0],
                vec![1, 4],
                vec![1, 8],
                vec![3, 5],
                vec![4, 6],
            ],
            1,
            2,
        ),
        (
            vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5],
                vec![6, 6],
            ],
            2,
            3,
        ),
        (vec![vec![2, 3], vec![1, 2]], 0, 2),
    ];

    for (points, w, ans) in tests {
        assert_eq!(Solution::min_rectangles_to_cover_points(points, w), ans);
    }
}
