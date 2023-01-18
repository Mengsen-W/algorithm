/*
 * @Date: 2022-12-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-04
 * @FilePath: /algorithm/1774_closest_cost/closest_cost.rs
 */

pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
    use std::cmp::min;
    let mut dp = vec![false; (target + 1) as usize];
    let mut ans = i32::MAX;
    for b in base_costs {
        if b > target {
            ans = min(ans, b);
            continue;
        }
        dp[b as usize] = true;
    }

    for t in &topping_costs.repeat(2) {
        let t = *t as usize;
        for i in (1..=target as usize).rev() {
            if dp[i] && (i + t) as i32 > target {
                ans = min(ans, (i + t) as i32);
            }
            if i >= t {
                dp[i] = dp[i] || dp[i - t];
            }
        }
    }

    for i in 0..=ans - target {
        if target >= i && dp[(target - i) as usize] {
            return target - i;
        }
    }
    return ans;
}

fn main() {
    {
        let base_costs = vec![1, 7];
        let topping_costs = vec![3, 4];
        let target = 10;
        let ans = 10;
        assert_eq!(closest_cost(base_costs, topping_costs, target), ans);
    }

    {
        let base_costs = vec![2, 3];
        let topping_costs = vec![4, 5, 100];
        let target = 18;
        let ans = 17;
        assert_eq!(closest_cost(base_costs, topping_costs, target), ans);
    }

    {
        let base_costs = vec![3, 10];
        let topping_costs = vec![2, 5];
        let target = 9;
        let ans = 8;
        assert_eq!(closest_cost(base_costs, topping_costs, target), ans);
    }

    {
        let base_costs = vec![10];
        let topping_costs = vec![1];
        let target = 1;
        let ans = 10;
        assert_eq!(closest_cost(base_costs, topping_costs, target), ans);
    }
}
