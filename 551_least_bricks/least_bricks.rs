/*
 * @Date: 2021-05-02 09:25:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-02 09:48:52
 */

fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let n = wall.len();
    let mut acc_map = HashMap::new();

    let mut acc = 0;
    for layer in wall {
        acc = 0;
        for brick in layer {
            acc += brick;
            let count = acc_map.entry(acc).or_insert(0);
            *count += 1;
        }
    }
    acc_map.remove(&acc);
    n as i32 - *acc_map.values().max().unwrap_or(&0)
}

fn main() {
    {
        let walls = vec![
            vec![1, 2, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 4],
            vec![3, 1, 2],
        ];
        assert_eq!(least_bricks(walls), 2);
    }
    {
        let walls = vec![vec![1], vec![1], vec![1]];
        assert_eq!(least_bricks(walls), 3);
    }
}
