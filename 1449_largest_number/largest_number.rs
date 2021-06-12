/*
 * @Date: 2021-06-12 09:53:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-12 11:41:09
 */

fn largest_number(cost: Vec<i32>, target: i32) -> String {
    let target = target as usize;
    let mut dp = vec![i32::MIN; target + 1];
    dp[0] = 0;

    for c in &cost {
        let c = *c as usize;
        for j in c..=target {
            dp[j] = dp[j].max(dp[(j - c)] + 1);
        }
    }

    if dp[target] < 0 {
        return "0".to_string();
    }

    let mut ans = String::new();
    let mut j = target;
    for i in (0..=8).rev() {
        let c = cost[i] as usize;
        while j >= c && dp[j] == dp[j - c] + 1 {
            ans.push_str(&((1 + i).to_string()));
            j -= c;
        }
    }
    ans
}

fn main() {
    {
        let cost = vec![4, 3, 2, 5, 6, 7, 2, 5, 5];
        let target = 9;
        let ans = "7772".to_string();
        assert_eq!(largest_number(cost, target), ans);
    }
    {
        let cost = vec![7, 6, 5, 5, 5, 6, 8, 7, 8];
        let target = 12;
        let ans = "85".to_string();
        assert_eq!(largest_number(cost, target), ans);
    }
    {
        let cost = vec![2, 4, 6, 2, 4, 6, 4, 4, 4];
        let target = 5;
        let ans = "0".to_string();
        assert_eq!(largest_number(cost, target), ans);
    }
    {
        let cost = vec![6, 10, 15, 40, 40, 40, 40, 40, 40];
        let target = 47;
        let ans = "32211".to_string();
        assert_eq!(largest_number(cost, target), ans);
    }
}
