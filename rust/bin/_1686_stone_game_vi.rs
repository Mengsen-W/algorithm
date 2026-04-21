/*
 * @Date: 2024-02-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-02
 * @FilePath: /algorithm/rust/1686_stone_game_vi/stone_game_vi.rs
 */

struct Solution;
impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut values: Vec<Vec<i32>> = alice_values
            .into_iter()
            .zip(bob_values)
            .map(|(a, b)| vec![a + b, a, b])
            .collect();

        values.sort_by(|a, b| b[0].cmp(&a[0]));
        let alice_sum: i32 = values.iter().step_by(2).map(|value| value[1]).sum();
        let bob_sum: i32 = values.iter().skip(1).step_by(2).map(|value| value[2]).sum();

        if alice_sum > bob_sum {
            1
        } else if alice_sum == bob_sum {
            0
        } else {
            -1
        }
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3], vec![2, 1], 1),
        (vec![1, 2], vec![3, 1], 0),
        (vec![2, 4, 3], vec![1, 6, 7], -1),
    ];

    for (alice_values, bob_values, ans) in tests {
        assert_eq!(Solution::stone_game_vi(alice_values, bob_values), ans);
    }
}
