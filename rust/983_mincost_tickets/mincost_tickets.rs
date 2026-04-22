struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let bisect = |x| match days.binary_search(&x) {
            Ok(r) => r + 1,
            Err(r) => r,
        };
        let mut ans = vec![0];
        for (i, day) in days.iter().enumerate() {
            ans.push(
                (ans[i] + costs[0])
                    .min(ans[bisect(day - 7)] + costs[1])
                    .min(ans[bisect(day - 30)] + costs[2]),
            )
        }
        *ans.last().unwrap() as i32
    }
}

fn main() {
    let tests = vec![
        (vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15], 11),
        (
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31],
            vec![2, 7, 15],
            17,
        ),
    ];

    for (days, costs, ans) in tests {
        assert_eq!(Solution::mincost_tickets(days, costs), ans);
    }
}
