/*
 * @Date: 2022-05-20 22:19:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-20 23:22:11
 * @FilePath: /algorithm/436_find_right_interval/find_right_interval.rs
 */

pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let (mut start_intervals, mut end_intervals) = intervals.iter().enumerate().fold(
        (Vec::new(), Vec::new()),
        |(mut a, mut b), (i, interval)| {
            a.push((interval[0], i));
            b.push((interval[1], i));
            (a, b)
        },
    );

    start_intervals.sort_unstable();
    end_intervals.sort_unstable();

    let n = intervals.len();
    let mut ans: Vec<i32> = vec![-1; n];
    let (mut i, mut j) = (0, 0);
    while i < n && j < n {
        while j < n && end_intervals[i].0 > start_intervals[j].0 {
            j += 1;
        }
        if j < n {
            ans[end_intervals[i].1] = start_intervals[j].1 as i32;
        }
        i += 1;
    }
    ans
}

fn main() {
    // assert_eq!(find_right_interval(vec![vec![1, 2]]), vec![-1]);
    // assert_eq!(
    //     find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
    //     vec![-1, 0, 1]
    // );
    assert_eq!(
        find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
        vec![-1, 2, -1]
    );
}
