struct Solution;

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();

        // 按照价格从低到高买
        for (i, &cost) in costs.iter().enumerate() {
            if coins < cost {
                // 钱不够
                return i as _; // 买 [0, i-1] 一共 i 根雪糕
            }
            coins -= cost;
        }

        // 可以买所有雪糕
        costs.len() as _
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 2, 4, 1], 7, 4),
        (vec![10, 6, 8, 7, 7, 8], 5, 0),
        (vec![1, 6, 3, 1, 2, 5], 20, 6),
    ];

    for (costs, coins, expected) in tests {
        assert_eq!(Solution::max_ice_cream(costs, coins), expected);
    }
}
