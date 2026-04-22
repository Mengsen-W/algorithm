struct Solution;

impl Solution {
    pub fn count_days(mut days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_by_key(|m| m[0]);
        let (mut l, mut r) = (1, 0);
        for m in meetings {
            let (start, end) = (m[0], m[1]);
            if start > r {
                days -= r - l + 1;
                l = start;
            }
            r = r.max(end);
        }
        days -= r - l + 1;
        days
    }
}

fn main() {
    let tests = vec![
        (10, vec![vec![5, 7], vec![1, 3], vec![9, 10]], 2),
        (5, vec![vec![2, 4], vec![1, 3]], 1),
        (6, vec![vec![1, 6]], 0),
    ];

    for (days, meetings, expected) in tests {
        assert_eq!(Solution::count_days(days, meetings), expected);
    }
}
