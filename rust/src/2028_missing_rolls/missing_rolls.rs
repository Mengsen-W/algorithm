/*
 * @Date: 2022-03-27 02:45:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-27 03:14:14
 * @FilePath: /algorithm/2028_missing_rolls/missing_rolls.rs
 */

pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let sum = (rolls.len() as i32 + n) * mean - rolls.iter().sum::<i32>();
    if sum < n || sum > 6 * n {
        return vec![];
    }
    [
        vec![sum / n; (n - sum % n) as usize],
        vec![sum / n + 1; (sum % n) as usize],
    ]
    .concat()
}

fn main() {
    {
        let rolls = vec![3, 2, 4, 3];
        let mean = 4;
        let n = 2;
        let ans = vec![6, 6];
        assert_eq!(missing_rolls(rolls, mean, n), ans);
    }

    {
        let rolls = vec![1, 5, 6];
        let mean = 3;
        let n = 4;
        let ans = vec![2, 2, 2, 3];
        assert_eq!(missing_rolls(rolls, mean, n), ans);
    }

    {
        let rolls = vec![1, 2, 3, 4];
        let mean = 5;
        let n = 4;
        let ans = vec![];
        assert_eq!(missing_rolls(rolls, mean, n), ans);
    }

    {
        let rolls = vec![1];
        let mean = 3;
        let n = 1;
        let ans = vec![5];
        assert_eq!(missing_rolls(rolls, mean, n), ans);
    }
}
