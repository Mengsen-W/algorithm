struct Solution;

impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut min1 = vec![1000000001; 26];
        let mut min2 = 1000000001;
        for (i, ch) in s.chars().enumerate() {
            let x = points[i][0];
            let y = points[i][1];
            let j = (ch as u8 - b'a') as usize;
            let d = x.abs().max(y.abs());
            if d < min1[j] {
                min2 = min2.min(min1[j]);
                min1[j] = d;
            } else if d < min2 {
                min2 = d;
            }
        }
        let mut res = 0;
        for d in min1 {
            if d < min2 {
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
                vec![2, 2],
                vec![-1, -2],
                vec![-4, 4],
                vec![-3, 1],
                vec![3, -3],
            ],
            "abdca",
            2,
        ),
        (vec![vec![1, 1], vec![-2, -2], vec![-2, 2]], "abb", 1),
        (vec![vec![1, 1], vec![-1, -1], vec![2, -2]], "ccd", 0),
    ];

    for (points, s, ans) in tests {
        assert_eq!(
            Solution::max_points_inside_square(points, s.to_string()),
            ans
        );
    }
}
