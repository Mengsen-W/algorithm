/*
 * @Date: 2023-07-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-19
 * @FilePath: /algorithm/rust/872_robot_sim/robot_sim.rs
 */

struct Solution;
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let (mut row_map, mut col_map): (HashMap<i32, Vec<i32>>, HashMap<i32, Vec<i32>>) =
            (HashMap::new(), HashMap::new());
        for place in obstacles.iter() {
            if place[0] == 0 && place[1] == 0 {
                continue;
            }
            // 向row_map[place[0]]插入place[1]
            let value = row_map.entry(place[0]).or_insert(vec![]);
            (*value).insert(value.partition_point(|&num| num < place[1]), place[1]);
            // 向col_map[place[1]]插入place[0]
            let value = col_map.entry(place[1]).or_insert(vec![]);
            (*value).insert(value.partition_point(|&num| num < place[0]), place[0]);
        }
        let (mut now_x, mut now_y, mut direct, mut res): (i32, i32, i32, i32) = (0, 0, 0, 0);
        for &order in commands.iter() {
            match order {
                -1 => direct = (direct + 1) % 4,
                -2 => direct = (direct + 3) % 4,
                _ => match direct {
                    0 => {
                        let value = row_map.entry(now_x).or_insert(vec![]);
                        let index = (*value).partition_point(|&num| num < now_y);
                        if index == (*value).len() {
                            // 直接往上走
                            now_y += order;
                        } else {
                            // 障碍
                            now_y = std::cmp::min(now_y + order, (*value)[index] - 1);
                        }
                    }
                    1 => {
                        let value = col_map.entry(now_y).or_insert(vec![]);
                        let index = (*value).partition_point(|&num| num < now_x);
                        if index == (*value).len() {
                            // 直接往右走
                            now_x += order;
                        } else {
                            // 障碍
                            now_x = std::cmp::min(now_x + order, (*value)[index] - 1);
                        }
                    }
                    2 => {
                        let value = row_map.entry(now_x).or_insert(vec![]);
                        let index = (*value).partition_point(|&num| num < now_y);
                        if index == 0 {
                            // 直接往下走
                            now_y -= order;
                        } else {
                            // 障碍
                            now_y = std::cmp::max(now_y - order, (*value)[index - 1] + 1);
                        }
                    }
                    _ => {
                        let value = col_map.entry(now_y).or_insert(vec![]);
                        let index = (*value).partition_point(|&num| num < now_x);
                        if index == 0 {
                            // 直接往左走
                            now_x -= order;
                        } else {
                            // 障碍
                            now_x = std::cmp::max(now_x - order, (*value)[index - 1] + 1);
                        }
                    }
                },
            }
            // println!("(x = {}, y = {}, direct = {})", now_x, now_y, direct);
            res = std::cmp::max(res, now_x.pow(2) + now_y.pow(2));
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![4, -1, 3], vec![], 25),
        (vec![4, -1, 4, -2, 4], vec![vec![2, 4]], 65),
    ];

    for (commands, obstacles, ans) in tests {
        assert_eq!(Solution::robot_sim(commands, obstacles), ans);
    }
}
