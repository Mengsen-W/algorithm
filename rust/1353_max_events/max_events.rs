struct Solution;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut events = events;
        events.sort_by(|a, b| a[0].cmp(&b[0]));
        let max_day = events.iter().map(|e| e[1]).max().unwrap_or(0);
        let mut pq = BinaryHeap::new();
        let mut ans = 0;
        let mut j = 0;
        for i in 1..=max_day {
            while j < events.len() && events[j][0] <= i {
                pq.push(Reverse(events[j][1]));
                j += 1;
            }
            while let Some(&Reverse(end)) = pq.peek() {
                if end < i {
                    pq.pop();
                } else {
                    break;
                }
            }
            if let Some(Reverse(_)) = pq.pop() {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 2], vec![2, 3], vec![3, 4]], 3),
        (vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]], 4),
    ];

    for (events, ans) in tests {
        assert_eq!(Solution::max_events(events), ans);
    }
}
