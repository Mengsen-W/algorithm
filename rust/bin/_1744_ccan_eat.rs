/*
 * @Date: 2021-06-01 09:00:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-01 09:18:03
 */

fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut pre_eat = vec![0 as i64; candies_count.len() + 1];
    let mut res = Vec::with_capacity(queries.len());
    for i in 1..candies_count.len() + 1 {
        pre_eat[i] = pre_eat[i - 1] + candies_count[i - 1] as i64;
    }

    for q in &queries {
        let days = q[1] as i64 + 1;
        // 每天吃一个糖果，按最少的了量去吃，如果直接吃到了 q[0]+1 类，就肯定吃不过 q[0]类.
        if days > pre_eat[q[0] as usize + 1] {
            res.push(false);
        } else if days * q[2] as i64 <= pre_eat[q[0] as usize] {
            //如果每天吃最多的，也只能吃到 q[0]-1 类的糖果，所以吃不过
            res.push(false);
        } else {
            res.push(true);
        }
    }

    res
}

fn main() {
    assert_eq!(
        can_eat(
            vec![7, 4, 5, 3, 8],
            vec![vec![0, 2, 2], vec![4, 2, 4], vec![2, 13, 1000000000]]
        ),
        vec![true, false, true]
    );
    assert_eq!(
        can_eat(
            vec![5, 2, 6, 4, 1],
            vec![
                vec![3, 1, 2],
                vec![4, 10, 3],
                vec![3, 10, 100],
                vec![4, 100, 30],
                vec![1, 3, 1]
            ]
        ),
        vec![false, true, true, false, false]
    );
}
