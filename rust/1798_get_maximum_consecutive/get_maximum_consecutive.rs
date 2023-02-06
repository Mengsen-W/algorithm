/*
 * @Date: 2023-02-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-04
 * @FilePath: /algorithm/rust/1798_get_maximum_consecutive/get_maximum_consecutive.rs
 */

pub fn get_maximum_consecutive(mut coins: Vec<i32>) -> i32 {
    coins.sort_unstable();
    coins
        .into_iter()
        .fold(1, |s, x| if s >= x { s + x } else { s })
}

fn main() {
    {
        let coins = vec![1, 3];
        let ans = 2;
        assert_eq!(get_maximum_consecutive(coins), ans);
    }

    {
        let coins = vec![1, 1, 1, 4];
        let ans = 8;
        assert_eq!(get_maximum_consecutive(coins), ans);
    }

    {
        let coins = vec![1, 4, 10, 3, 1];
        let ans = 20;
        assert_eq!(get_maximum_consecutive(coins), ans);
    }
}
