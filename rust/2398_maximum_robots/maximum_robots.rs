struct Solution;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        use std::collections::VecDeque;
        let mut res = 0;
        let mut n = charge_times.len();
        let mut running_cost_sum = 0i64;
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut j = 0;
        for i in 0..n {
            running_cost_sum += running_costs[i] as i64;
            while !q.is_empty() && charge_times[*q.back().unwrap()] <= charge_times[i] {
                q.pop_back();
            }
            q.push_back(i);
            while j <= i
                && (i - j + 1) as i64 * running_cost_sum + charge_times[*q.front().unwrap()] as i64
                    > budget
            {
                if !q.is_empty() && *q.front().unwrap() == j {
                    q.pop_front();
                }
                running_cost_sum -= running_costs[j] as i64;
                j += 1;
            }
            res = res.max(i - j + 1);
        }
        res as i32
    }
}

fn main() {
    let tests = vec![
        (vec![3, 6, 1, 3, 4], vec![2, 1, 3, 4, 5], 25, 3),
        (vec![11, 12, 19], vec![10, 8, 7], 19, 0),
    ];

    for (charge_times, running_costs, budget, ans) in tests {
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            ans
        );
    }
}
