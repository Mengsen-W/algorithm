struct Solution;

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let n = points.len();

        for i in 0..n {
            let point_a = &points[i];
            for j in 0..n {
                let point_b = &points[j];
                if i == j || !(point_a[0] <= point_b[0] && point_a[1] >= point_b[1]) {
                    continue;
                }
                if n == 2 {
                    ans += 1;
                    continue;
                }

                let mut illegal = false;
                for k in 0..n {
                    if k == i || k == j {
                        continue;
                    }

                    let point_tmp = &points[k];
                    let is_x_contained = point_tmp[0] >= point_a[0] && point_tmp[0] <= point_b[0];
                    let is_y_contained = point_tmp[1] <= point_a[1] && point_tmp[1] >= point_b[1];
                    if is_x_contained && is_y_contained {
                        illegal = true;
                        break;
                    }
                }
                if !illegal {
                    ans += 1;
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
