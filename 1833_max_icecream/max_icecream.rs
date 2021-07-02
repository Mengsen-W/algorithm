/*
 * @Date: 2021-07-02 14:57:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-02 15:56:30
 */

fn max_ice_cream(costs: Vec<i32>, mut coins: i32) -> i32 {
    let mut freq = vec![0; 100001];
    for cost in costs {
        freq[cost as usize] += 1;
    }
    let mut count = 0;
    for i in 1..=100000 {
        if coins >= i {
            let cur_count = freq[i as usize].min(coins / i);
            count += cur_count;
            coins -= i * cur_count;
        } else {
            break;
        }
    }
    count
}

fn main() {
    {
        let costs = vec![1, 3, 2, 4, 1];
        let coins = 7;
        assert_eq!(max_ice_cream(costs, coins), 4);
    }
    {
        let costs = vec![10, 6, 8, 7, 7, 8];
        let coins = 5;
        assert_eq!(max_ice_cream(costs, coins), 0);
    }
    {
        let costs = vec![1, 6, 3, 1, 2, 5];
        let coins = 20;
        assert_eq!(max_ice_cream(costs, coins), 6);
    }
}
