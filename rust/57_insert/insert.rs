/*
 * @Date: 2023-08-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-28
 * @FilePath: /algorithm/rust/57_insert/insert.rs
 */

struct Solution;
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let (mut beg, mut end) = (new_interval[0], Some(new_interval[1]));

        for interval in intervals.into_iter() {
            match end {
                Some(end_range) => match end_range.cmp(&interval[0]) {
                    std::cmp::Ordering::Less => {
                        result.push(vec![beg, end_range]);
                        result.push(interval);
                        end = None;
                    }
                    std::cmp::Ordering::Equal => {
                        result.push(vec![beg, interval[1]]);
                        end = None;
                    }
                    std::cmp::Ordering::Greater => {
                        if end_range <= interval[1] {
                            result.push(vec![i32::min(beg, interval[0]), interval[1]]);
                            end = None;
                        } else if beg > interval[1] {
                            result.push(interval);
                        } else {
                            beg = i32::min(beg, interval[0]);
                        }
                    }
                },
                None => result.push(interval),
            }
        }
        if end.is_some() {
            result.push(vec![beg, end.unwrap()]);
        }

        result
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![2, 5],
            vec![vec![1, 5], vec![6, 9]],
        ),
        (
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            vec![4, 8],
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
        ),
        (vec![], vec![5, 7], vec![vec![5, 7]]),
        (vec![vec![1, 5]], vec![2, 3], vec![vec![1, 5]]),
        (vec![vec![1, 5]], vec![2, 7], vec![vec![1, 7]]),
    ];

    for (intervals, new_interval, result) in tests {
        assert_eq!(Solution::insert(intervals, new_interval), result);
    }
}
