/*
 * @Date: 2023-07-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-07
 * @FilePath: /algorithm/rust/2532_find_crossing_time/find_crossing_time.rs
 */

struct Solution;
impl Solution {
    pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut left = BinaryHeap::<(i32, usize)>::new();
        let mut right = BinaryHeap::<(i32, usize)>::new();
        let mut left_wait = BinaryHeap::<Reverse<(i32, usize)>>::new();
        let mut right_wait = BinaryHeap::<Reverse<(i32, usize)>>::new();

        for i in 0..k as usize {
            left.push((time[i][0] + time[i][2], i));
        }
        let (mut tm, mut n) = (0, n);
        loop {
            while let Some(Reverse((t, i))) = left_wait.peek() {
                if *t > tm {
                    break;
                }
                left.push((time[*i][0] + time[*i][2], *i));
                left_wait.pop();
            }

            while let Some(Reverse((t, i))) = right_wait.peek() {
                if *t > tm {
                    break;
                }
                right.push((time[*i][0] + time[*i][2], *i));
                right_wait.pop();
            }

            if let Some((_, i)) = right.pop() {
                tm += time[i][2];
                left_wait.push(Reverse((tm + time[i][3], i)));
                continue;
            }
            if n == 0 && right.is_empty() && right_wait.is_empty() {
                break;
            }

            if let Some((_, i)) = left.pop() {
                if n > 0 {
                    n -= 1;
                    tm += time[i][0];
                    right_wait.push(Reverse((tm + time[i][1], i)));
                    continue;
                }
            }

            let mut new_time = i32::MAX;
            if let Some(Reverse((t, _))) = left_wait.peek() {
                if n > 0 {
                    new_time = new_time.min(*t);
                }
            }
            if let Some(Reverse((t, _))) = right_wait.peek() {
                new_time = new_time.min(*t);
            }
            tm = tm.max(new_time);
        }
        tm
    }
}

fn main() {
    let test_map = vec![
        (
            1,
            3,
            vec![vec![1, 1, 2, 1], vec![1, 1, 3, 1], vec![1, 1, 4, 1]],
            6,
        ),
        (3, 2, vec![vec![1, 9, 1, 8], vec![10, 10, 10, 10]], 50),
    ];

    for (n, k, time, ans) in test_map {
        assert_eq!(Solution::find_crossing_time(n, k, time), ans);
    }
}
