struct Solution;

#[derive(Debug)]
struct Event {
    ts: i32,
    op: i32,
    val: i32,
}

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut evs: Vec<Event> = Vec::new();
        for event in events {
            evs.push(Event {
                ts: event[0],
                op: 0,
                val: event[2],
            });
            evs.push(Event {
                ts: event[1],
                op: 1,
                val: event[2],
            });
        }

        evs.sort_by(|a, b| {
            if a.ts != b.ts {
                a.ts.cmp(&b.ts)
            } else {
                a.op.cmp(&b.op)
            }
        });

        let mut ans = 0;
        let mut best_first = 0;
        for ev in evs {
            if ev.op == 0 {
                ans = ans.max(ev.val + best_first);
            } else {
                best_first = best_first.max(ev.val);
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]], 4),
        (vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]], 5),
        (vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]], 8),
    ];

    for (events, expected) in tests {
        assert_eq!(Solution::max_two_events(events), expected);
    }
}
