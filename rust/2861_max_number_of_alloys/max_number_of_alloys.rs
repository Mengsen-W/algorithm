/*
 * @Date: 2024-01-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-27
 * @FilePath: /algorithm/rust/2861_max_number_of_alloys/max_number_of_alloys.rs
 */

struct Solution;

impl Solution {
    pub fn max_number_of_alloys(
        n: i32,
        _: i32,
        budget: i32,
        composition: Vec<Vec<i32>>,
        stock: Vec<i32>,
        cost: Vec<i32>,
    ) -> i32 {
        let mx = *stock.iter().min().unwrap() as i32 + budget / n;
        let check = |num| -> bool {
            composition.iter().any(|comp| {
                let mut money = 0 as i64;
                for (j, &v) in comp.iter().enumerate() {
                    if v as i64 * num > stock[j] as i64 {
                        money += (v as i64 * num - stock[j] as i64) * cost[j] as i64;
                    }
                }
                money <= budget as i64
            })
        };

        let (mut l, mut r) = (0, mx);
        while l < r {
            let mid = (l + r + 1) >> 1;
            if check(mid as i64) {
                l = mid
            } else {
                r = mid - 1
            }
        }
        l
    }
}

fn main() {
    let tests = vec![
        (
            3,
            2,
            15,
            vec![vec![1, 1, 1], vec![1, 1, 10]],
            vec![0, 0, 0],
            vec![1, 2, 3],
            2,
        ),
        (
            3,
            2,
            15,
            vec![vec![1, 1, 1], vec![1, 1, 10]],
            vec![0, 0, 100],
            vec![1, 2, 3],
            5,
        ),
        (
            2,
            3,
            10,
            vec![vec![2, 1], vec![1, 2], vec![1, 1]],
            vec![1, 1],
            vec![5, 5],
            2,
        ),
    ];

    for (n, k, budget, composition, stock, cost, ans) in tests {
        assert_eq!(
            Solution::max_number_of_alloys(n, k, budget, composition, stock, cost),
            ans
        );
    }
}
