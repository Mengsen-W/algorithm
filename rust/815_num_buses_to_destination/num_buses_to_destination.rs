/*
 * @Date: 2021-06-28 08:36:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-28 09:02:04
 */

use std::collections::HashMap;

pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    let f = |table: HashMap<i32, i32>, i: &Vec<i32>| {
        if let Some(&v) = i.iter().filter_map(|e| table.get(e)).min() {
            i.iter().fold(table, |mut table, j| {
                if let Some(x) = table.get_mut(j) {
                    *x = (*x).min(v + 1);
                } else {
                    table.insert(*j, v + 1);
                }
                table
            })
        } else {
            table
        }
    };
    routes
        .iter()
        .fold(
            routes.iter().fold(
                [(source, 0)].iter().cloned().collect::<HashMap<i32, i32>>(),
                f,
            ),
            f,
        )
        .get(&target)
        .cloned()
        .unwrap_or(-1)
}

fn main() {
    {
        let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
        let source = 1;
        let target = 6;
        assert_eq!(num_buses_to_destination(routes, source, target), 2);
    }
    {
        let routes = vec![
            vec![7, 12],
            vec![4, 5, 15],
            vec![6],
            vec![15, 19],
            vec![9, 12, 13],
        ];
        let source = 15;
        let target = 12;
        assert_eq!(num_buses_to_destination(routes, source, target), -1);
    }
}
