struct Solution;

use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Item {
    value: i32,
    i: usize,
    j: usize,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let m = values.len();
        let n = values[0].len();
        let mut pq = BinaryHeap::new();
        for i in 0..m {
            pq.push(Item {
                value: values[i][n - 1],
                i,
                j: n - 1,
            });
        }

        let mut ans: i64 = 0;
        for turn in 1..=m * n {
            if let Some(top) = pq.pop() {
                let val = top.value;
                let i = top.i;
                let j = top.j;
                ans += (val as i64) * (turn as i64);
                if j > 0 {
                    pq.push(Item {
                        value: values[i][j - 1],
                        i,
                        j: j - 1,
                    });
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![8, 5, 2], vec![6, 4, 1], vec![9, 7, 3]], 285),
        (vec![vec![10, 8, 6, 4, 2], vec![9, 7, 5, 3, 2]], 386),
    ];

    for (values, ans) in tests {
        assert_eq!(Solution::max_spending(values), ans);
    }
}
