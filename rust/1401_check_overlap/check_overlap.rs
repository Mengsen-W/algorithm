struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let mut dist = 0;
        if x_center < x1 || x_center > x2 {
            dist += (x1 - x_center).pow(2).min((x2 - x_center).pow(2));
        }

        if y_center < y1 || y_center > y2 {
            dist += (y1 - y_center).pow(2).min((y2 - y_center).pow(2));
        }
        dist <= radius.pow(2)
    }
}

fn main() {
    let tests = vec![
        (1, 0, 0, 1, -1, 3, 1, true),
        (1, 1, 1, 1, -3, 2, -1, false),
        (1, 0, 0, -1, 0, 0, 1, true),
    ];

    for (radius, x_center, y_center, x1, y1, x2, y2, ans) in tests {
        assert_eq!(
            Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
            ans
        );
    }
}
