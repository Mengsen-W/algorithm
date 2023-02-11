/*
 * @Date: 2023-02-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-11
 * @FilePath: /algorithm/rust/2335_fill_cups/fill_cups.rs
 */

pub fn fill_cups(amount: Vec<i32>) -> i32 {
    amount
        .iter()
        .cloned()
        .max()
        .unwrap_or(0)
        .max((amount.iter().sum::<i32>() + 1) / 2)
}

fn main() {
    {
        let amount = vec![1, 4, 2];
        let ans = 4;
        assert_eq!(fill_cups(amount), ans);
    }

    {
        let amount = vec![5, 4, 4];
        let ans = 7;
        assert_eq!(fill_cups(amount), ans);
    }

    {
        let amount = vec![5, 0, 0];
        let ans = 5;
        assert_eq!(fill_cups(amount), ans);
    }
}
