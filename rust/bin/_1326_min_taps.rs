/*
 * @Date: 2023-02-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-21
 * @FilePath: /algorithm/rust/1326_min_taps/min_taps.rs
 */

pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
    let mut ranges = ranges
        .iter()
        .enumerate()
        .map(|(i, r)| ((i as i32 - r).max(0), i as i32 + r))
        .collect::<Vec<_>>();

    ranges.sort_by_key(|(a, _)| *a);

    let mut ans = vec![];
    ans.push(ranges[0]);

    for r in ranges[1..].into_iter() {
        let len = ans.len();
        if r.0 <= ans[len - 1].0 && r.1 > ans[len - 1].1 {
            ans[len - 1].1 = r.1
        } else if r.0 <= ans[len - 1].1 && r.1 > ans[len - 1].1 {
            ans.push((ans[len - 1].1, r.1));
        }
        let len = ans.len();
        if ans[len - 1].1 >= n {
            return len as i32;
        }
    }
    -1
}

fn main() {
    {
        let n = 5;
        let ranges = vec![3, 4, 1, 1, 0, 0];
        let ans = 1;
        assert_eq!(min_taps(n, ranges), ans);
    }

    {
        let n = 3;
        let ranges = vec![0, 0, 0, 0];
        let ans = -1;
        assert_eq!(min_taps(n, ranges), ans);
    }
}
