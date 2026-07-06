struct Solution;

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        let n = intervals.len();
        intervals.sort_by(|u, v| {
            if u[0] != v[0] {
                u[0].cmp(&v[0])
            } else {
                v[1].cmp(&u[1])
            }
        });
        let mut ans = n as i32;
        let mut rmax = intervals[0][1];
        for i in 1..n {
            if intervals[i][1] <= rmax {
                ans -= 1;
            } else {
                rmax = rmax.max(intervals[i][1]);
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![vec![1, 4], vec![3, 6], vec![2, 8]], 2)];

    for (intervals, expected) in tests {
        assert_eq!(Solution::remove_covered_intervals(intervals), expected);
    }
}
