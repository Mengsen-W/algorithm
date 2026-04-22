/*
 * @Date: 2023-12-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-27
 * @FilePath: /algorithm/rust/2660_is_winner/is_winner.rs
 */

struct Solution;

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let f = |arr: &Vec<i32>| -> i32 {
            let mut s = 0;
            for i in 0..arr.len() {
                let mut k = 1;
                if (i > 0 && arr[i - 1] == 10) || (i > 1 && arr[i - 2] == 10) {
                    k = 2;
                }
                s += k * arr[i];
            }
            s
        };

        let a = f(&player1);
        let b = f(&player2);
        if a > b {
            1
        } else if a < b {
            2
        } else {
            0
        }
    }
}

fn main() {
    let tests = vec![
        (vec![4, 10, 7, 9], vec![6, 5, 2, 3], 1),
        (vec![3, 5, 7, 6], vec![8, 10, 10, 2], 2),
        (vec![2, 3], vec![4, 1], 0),
    ];

    for (player1, player2, ans) in tests {
        assert_eq!(Solution::is_winner(player1, player2), ans);
    }
}
