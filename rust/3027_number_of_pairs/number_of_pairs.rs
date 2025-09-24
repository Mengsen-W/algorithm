struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut col: HashMap<i32, usize> = HashMap::new();
        let mut row: HashMap<i32, usize> = HashMap::new();
        let mut coordinates_map: HashMap<(i32, i32), (usize, usize)> = HashMap::new();

        for point in &points {
            let x = point[0];
            let y = point[1];
            col.insert(x, 0);
            row.insert(y, 0);
        }

        let mut col_keys: Vec<i32> = col.keys().cloned().collect();
        col_keys.sort();
        for (idx, &key) in col_keys.iter().enumerate() {
            col.insert(key, idx + 1);
        }
        let mut row_keys: Vec<i32> = row.keys().cloned().collect();
        row_keys.sort();
        for (idx, &key) in row_keys.iter().enumerate() {
            row.insert(key, idx + 1);
        }

        let nc = col.len() + 1;
        let nr = row.len() + 1;
        let mut m = vec![vec![0; nr]; nc];
        let mut prefix_sum = vec![vec![0; nr]; nc];

        for point in &points {
            let x = point[0];
            let y = point[1];
            let c = *col.get(&x).unwrap();
            let r = *row.get(&y).unwrap();
            coordinates_map.insert((x, y), (c, r));
            m[c][r] = 1;
        }

        for i in 1..nc {
            for j in 1..nr {
                prefix_sum[i][j] = prefix_sum[i - 1][j] + prefix_sum[i][j - 1]
                    - prefix_sum[i - 1][j - 1]
                    + m[i][j];
            }
        }

        let mut ans = 0;
        let mut sorted_points = points.clone();

        sorted_points.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1]) // 降序
            } else {
                a[0].cmp(&b[0]) // 升序
            }
        });

        let n = sorted_points.len();
        for i in 0..n - 1 {
            for j in i + 1..n {
                if sorted_points[i][1] >= sorted_points[j][1] {
                    let (x1, y1) = (sorted_points[i][0], sorted_points[i][1]);
                    let (x2, y2) = (sorted_points[j][0], sorted_points[j][1]);
                    let (c1, r1) = coordinates_map[&(x1, y1)];
                    let (c2, r2) = coordinates_map[&(x2, y2)];

                    let cnt = prefix_sum[c2][r1] - prefix_sum[c1 - 1][r1] - prefix_sum[c2][r2 - 1]
                        + prefix_sum[c1 - 1][r2 - 1];

                    if cnt == 2 {
                        ans += 1;
                    }
                }
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 1], vec![2, 2], vec![3, 3]], 0),
        (vec![vec![6, 2], vec![4, 4], vec![2, 6]], 2),
        (vec![vec![3, 1], vec![1, 3], vec![1, 1]], 2),
    ];

    for (points, expected) in tests {
        assert_eq!(Solution::number_of_pairs(points), expected);
    }
}
