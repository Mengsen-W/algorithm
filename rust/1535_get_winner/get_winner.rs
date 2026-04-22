/*
 * @Date: 2024-05-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-19
 * @FilePath: /algorithm/rust/1535_get_winner/get_winner.rs
 */

struct Solution;

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut prev = arr[0].max(arr[1]);
        if k == 1 {
            return prev;
        }
        let mut consecutive = 1;
        let mut max_num = prev;
        for &num in &arr[2..] {
            let curr = num;
            if prev > curr {
                consecutive += 1;
                if consecutive == k {
                    return prev;
                }
            } else {
                prev = curr;
                consecutive = 1;
            }
            max_num = max_num.max(curr);
        }
        max_num
    }
}

fn main() {
    let tests = vec![
        (vec![2, 1, 3, 5, 4, 6, 7], 2, 5),
        (vec![3, 2, 1], 10, 3),
        (vec![1, 9, 8, 2, 3, 7, 6, 4, 5], 7, 9),
        (vec![1, 11, 22, 33, 44, 55, 66, 77, 88, 99], 1000000000, 99),
    ];

    for (arr, k, ans) in tests {
        assert_eq!(Solution::get_winner(arr, k), ans);
    }
}
