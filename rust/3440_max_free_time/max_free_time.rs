struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut q = vec![false; n];
        let mut t1 = 0;
        let mut t2 = 0;
        for i in 0..n {
            if end_time[i] - start_time[i] <= t1 {
                q[i] = true;
            }
            t1 = t1.max(start_time[i] - if i == 0 { 0 } else { end_time[i - 1] });

            let j = n - i - 1;
            if end_time[j] - start_time[j] <= t2 {
                q[j] = true;
            }
            t2 = t2.max(
                if i == 0 {
                    event_time
                } else {
                    start_time[n - i]
                } - end_time[j],
            );
        }

        let mut res = 0;
        for i in 0..n {
            let left = if i == 0 { 0 } else { end_time[i - 1] };
            let right = if i == n - 1 {
                event_time
            } else {
                start_time[i + 1]
            };
            if q[i] {
                res = res.max(right - left);
            } else {
                res = res.max(right - left - (end_time[i] - start_time[i]));
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (5, vec![1, 3], vec![2, 5], 2),
        (10, vec![0, 7, 9], vec![1, 8, 10], 7),
        (10, vec![0, 3, 7, 9], vec![1, 4, 8, 10], 6),
        (5, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5], 0),
    ];

    for (event_time, start_time, end_time, expected) in tests {
        assert_eq!(
            Solution::max_free_time(event_time, start_time, end_time),
            expected
        );
    }
}
