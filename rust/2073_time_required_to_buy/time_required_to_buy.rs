struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        use std::convert::TryInto;
        let n = tickets.len() as i32;
        let mut res = 0;
        for i in 0..n {
            // 遍历计算每个人所需时间
            if i <= k.try_into().unwrap() {
                res += std::cmp::min(tickets[i as usize], tickets[k as usize]);
            } else {
                res += std::cmp::min(tickets[i as usize], tickets[k as usize] - 1);
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![2, 3, 2], 2, 6), (vec![5, 1, 1, 1], 0, 8)];

    for (tickets, k, ans) in tests {
        assert_eq!(Solution::time_required_to_buy(tickets, k), ans);
    }
}
