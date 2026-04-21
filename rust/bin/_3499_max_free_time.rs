struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let k = k as usize;
        let mut res: i32 = 0;
        let mut t: i32 = 0;

        for i in 0..n {
            t += end_time[i] - start_time[i];
            let left = if i <= k - 1 { 0 } else { end_time[i - k] };
            let right = if i == n - 1 {
                event_time
            } else {
                start_time[i + 1]
            };
            res = res.max(right - left - t);
            if i >= k - 1 {
                t -= end_time[i - k + 1] - start_time[i - k + 1];
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (5, 1, vec![1, 3], vec![2, 5], 2),
        (10, 1, vec![0, 2, 9], vec![1, 4, 10], 6),
        (5, 2, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5], 0),
    ];

    for (event_time, k, start_time, end_time, expected) in tests {
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            expected
        );
    }
}
