struct Solution;

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let mut total_cost: i64 = 0;
        let mut need1 = need1;
        let mut need2 = need2;

        // 分别购买更便宜，不需要物品 3
        if cost1 + cost2 <= cost_both {
            total_cost = cost1 as i64 * need1 as i64 + cost2 as i64 * need2 as i64;
        } else {
            // 优先购买物品 3，直到一种需求满足
            let cnt_3 = need1.min(need2);
            total_cost = cnt_3 as i64 * cost_both as i64;

            need1 -= cnt_3;
            need2 -= cnt_3;

            // 类型 1 的需求还未被满足，选择物品 1 或物品 3 中更便宜的
            if need1 > 0 {
                total_cost += need1 as i64 * cost1.min(cost_both) as i64;
            }
            // 类型 2 的需求还未被满足，选择物品 2 或物品 3 中更便宜的
            else if need2 > 0 {
                total_cost += need2 as i64 * cost2.min(cost_both) as i64;
            }
        }

        total_cost
    }
}

fn main() {
    let tests = vec![
        (3, 2, 1, 3, 2, 3),
        (5, 4, 15, 2, 3, 22),
        (5, 4, 15, 0, 0, 0),
    ];

    for (cost1, cost2, cost_both, need1, need2, ans) in tests {
        assert_eq!(
            Solution::minimum_cost(cost1, cost2, cost_both, need1, need2),
            ans
        );
    }
}
