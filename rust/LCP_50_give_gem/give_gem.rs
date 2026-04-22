/*
 * @Date: 2023-09-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-15
 * @FilePath: /algorithm/rust/LCP_50_give_gem/give_gem.rs
 */

struct Solution;
impl Solution {
    pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        for op in operations {
            let num = gem[op[0] as usize] / 2;
            gem[op[0] as usize] -= num;
            gem[op[1] as usize] += num;
        }

        *gem.iter().max().unwrap() - *gem.iter().min().unwrap()
    }
}

fn main() {
    let tests = vec![
        (vec![3, 1, 2], vec![vec![0, 2], vec![2, 1], vec![2, 0]], 2),
        (
            vec![100, 0, 50, 100],
            vec![vec![0, 2], vec![0, 1], vec![3, 0], vec![3, 0]],
            75,
        ),
        (
            vec![0, 0, 0, 0],
            vec![vec![1, 2], vec![3, 1], vec![1, 2]],
            0,
        ),
    ];

    for (gem, operations, ans) in tests {
        assert_eq!(Solution::give_gem(gem, operations), ans);
    }
}
