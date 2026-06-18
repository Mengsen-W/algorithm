struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let one_min_angle = 6.0;
        let one_hour_angle = 30.0;

        let minutes_angle = one_min_angle * minutes as f64;
        let hour_angle = (hour % 12) as f64 + (minutes as f64 / 60.0);
        let hour_angle = hour_angle * one_hour_angle;

        let diff = (hour_angle - minutes_angle).abs();
        diff.min(360.0 - diff)
    }
}

fn main() {
    let tests = vec![
        (12, 30, 165.0),
        (3, 30, 75.0),
        (3, 15, 7.5),
        (4, 50, 155.0),
        (12, 0, 0.0),
    ];

    for (hour, minutes, expected) in tests {
        assert_eq!(Solution::angle_clock(hour, minutes), expected);
    }
}
