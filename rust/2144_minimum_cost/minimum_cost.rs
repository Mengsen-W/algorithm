struct Solution;

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_by(|a, b| b.cmp(a));
        let mut res = 0;
        let n = cost.len();
        for i in 0..n {
            if i % 3 != 2 {
                res += cost[i];
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], 5),
        (vec![6, 5, 7, 9, 2, 2], 23),
        (vec![5, 5], 10),
    ];

    for (cost, expected) in tests {
        assert_eq!(Solution::minimum_cost(cost), expected);
    }
}
