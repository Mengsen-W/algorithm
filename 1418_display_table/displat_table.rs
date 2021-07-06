/*
 * @Date: 2021-07-06 08:59:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-06 20:35:20
 */

use std::collections::BTreeMap;
use std::str::FromStr;

pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut name_map = BTreeMap::new();
    orders.iter().for_each(|order| {
        if !name_map.contains_key(&order[2]) {
            name_map.insert(order[2].clone(), 0);
        }
    });
    name_map
        .iter_mut()
        .enumerate()
        .for_each(|(i, (_, x))| *x = i);
    let mut order_map = BTreeMap::new();
    orders.into_iter().for_each(|order| {
        order_map
            .entry(i32::from_str(&order[1]).unwrap())
            .or_insert_with(|| vec![0; name_map.len()])[name_map[&order[2]]] += 1;
    });
    let mut title = vec!["Table".to_owned()];
    title.append(
        &mut name_map
            .into_iter()
            .map(|(name, _)| name)
            .collect::<Vec<String>>(),
    );
    let mut ans = vec![title];
    order_map.into_iter().for_each(|(id, v)| {
        let mut order = vec![id.to_string()];
        order.append(
            &mut v
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
        ans.push(order);
    });
    ans
}

fn main() {
    {
        let orders = vec![
            vec!["David".to_string(), "3".to_string(), "Ceviche".to_string()],
            vec![
                "Corina".to_string(),
                "10".to_string(),
                "Beef Burrito".to_string(),
            ],
            vec![
                "David".to_string(),
                "3".to_string(),
                "Fried Chicken".to_string(),
            ],
            vec!["Carla".to_string(), "5".to_string(), "Water".to_string()],
            vec!["Carla".to_string(), "5".to_string(), "Ceviche".to_string()],
            vec!["Rous".to_string(), "3".to_string(), "Ceviche".to_string()],
        ];
        let ans = vec![
            vec![
                "Table".to_string(),
                "Beef Burrito".to_string(),
                "Ceviche".to_string(),
                "Fried Chicken".to_string(),
                "Water".to_string(),
            ],
            vec![
                "3".to_string(),
                "0".to_string(),
                "2".to_string(),
                "1".to_string(),
                "0".to_string(),
            ],
            vec![
                "5".to_string(),
                "0".to_string(),
                "1".to_string(),
                "0".to_string(),
                "1".to_string(),
            ],
            vec![
                "10".to_string(),
                "1".to_string(),
                "0".to_string(),
                "0".to_string(),
                "0".to_string(),
            ],
        ];
        assert_eq!(display_table(orders), ans);
    }
    {
        let orders = vec![
            vec![
                "James".to_string(),
                "12".to_string(),
                "Fried Chicken".to_string(),
            ],
            vec![
                "Ratesh".to_string(),
                "12".to_string(),
                "Fried Chicken".to_string(),
            ],
            vec![
                "Amadeus".to_string(),
                "12".to_string(),
                "Fried Chicken".to_string(),
            ],
            vec![
                "Adam".to_string(),
                "1".to_string(),
                "Canadian Waffles".to_string(),
            ],
            vec![
                "Brianna".to_string(),
                "1".to_string(),
                "Canadian Waffles".to_string(),
            ],
        ];
        let ans = vec![
            vec![
                "Table".to_string(),
                "Canadian Waffles".to_string(),
                "Fried Chicken".to_string(),
            ],
            vec!["1".to_string(), "2".to_string(), "0".to_string()],
            vec!["12".to_string(), "0".to_string(), "3".to_string()],
        ];
        assert_eq!(display_table(orders), ans);
    }
    {
        let orders = vec![
            vec![
                "Laura".to_string(),
                "2".to_string(),
                "Bean Burrito".to_string(),
            ],
            vec![
                "Jhon".to_string(),
                "2".to_string(),
                "Beef Burrito".to_string(),
            ],
            vec!["Melissa".to_string(), "2".to_string(), "Soda".to_string()],
        ];
        let ans = vec![
            vec![
                "Table".to_string(),
                "Bean Burrito".to_string(),
                "Beef Burrito".to_string(),
                "Soda".to_string(),
            ],
            vec![
                "2".to_string(),
                "1".to_string(),
                "1".to_string(),
                "1".to_string(),
            ],
        ];
        assert_eq!(display_table(orders), ans);
    }
}
