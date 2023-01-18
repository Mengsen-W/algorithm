/*
 * @Date: 2022-07-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-22
 * @FilePath: /algorithm/757_intersection_size_two/intersection_size_two.rs
 */

pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.len() == 1 {
        return 2;
    }

    intervals.sort_by_key(|range| (range[1], -range[0]));
    intervals
        .into_iter()
        .map(|range| (range[0], range[1]))
        .fold((-1, -1, 0), |(a, b, ans), (l, r)| {
            if b < l {
                (r - 1, r, ans + 2)
            } else if a < l {
                (b, r, ans + 1)
            } else {
                (a, b, ans)
            }
        })
        .2
}

fn main() {
    {
        let intervals = vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]];
        assert_eq!(intersection_size_two(intervals), 3);
    }

    {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]];
        assert_eq!(intersection_size_two(intervals), 5);
    }
}
