/*
 * @Date: 2022-03-14 00:48:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-14 01:31:16
 * @FilePath: /algorithm/599_find_restaurant/find_restaurant.rs
 */

pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    use std::collections::HashMap;
    let hm: HashMap<_, _> = list1.iter().enumerate().map(|e| (e.1, e.0)).collect();
    let cons: Vec<_> = list2
        .iter()
        .enumerate()
        .filter(|e| hm.get(e.1).is_some())
        .map(|e| (e.0 + hm.get(e.1).unwrap(), e.1))
        .collect();
    let min_idx_sum =
        cons.iter().map(|e| e.0).fold(
            usize::max_value(),
            |acc, cur| if cur < acc { cur } else { acc },
        );
    cons.iter()
        .filter(|e| e.0 == min_idx_sum)
        .map(|e| e.1.clone())
        .collect::<Vec<_>>()
}

fn main() {
    assert_eq!(
        find_restaurant(
            vec!["Shogun", "Tapioca Express", "Burger King", "KFC"]
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            vec![
                "Piatti",
                "The Grill at Torrey Pines",
                "Hungry Hunter Steakhouse",
                "Shogun"
            ]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
        ),
        vec!["Shogun"]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        find_restaurant(
            vec!["Shogun", "Tapioca Express", "Burger King", "KFC"]
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            vec!["KFC", "Shogun", "Burger King"]
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
        ),
        vec!["Shogun"]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
    );
}
