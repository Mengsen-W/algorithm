/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-19 09:46:53
 * @Last Modified by:   Mengsen.Wang
 * @Last Modified time: 2021-01-19 09:46:53
 */

fn just_recursive(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }

    just_recursive(n - 1) + just_recursive(n - 2)
}

fn dp_fib(n: i32) -> i32 {
    let mut dp: Vec<i32> = vec![0; n as usize + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 3..=n {
        dp[i as usize] = dp[i as usize - 1] + dp[i as usize - 2];
    }
    dp[n as usize]
}
fn fib(n: i32) -> i32 {
    (0..n).fold((0, 1), |(pp, p), _| (p, pp + p)).0
}

fn main() {
    println!("The result = {}", just_recursive(45));
    println!("The result = {}", dp_fib(45));
    println!("The result = {}", fib(45));
}
