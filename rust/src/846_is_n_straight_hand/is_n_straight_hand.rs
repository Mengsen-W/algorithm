/*
 * @Date: 2021-12-30 01:18:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-30 01:45:47
 */

pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
    use std::collections::HashMap;
    hand.sort_unstable();
    let n = hand.len() as i32;
    if n % group_size != 0 {
        return false;
    }
    let mut map = hand.clone().into_iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    for x in &hand {
        if !map.contains_key(&x) {
            continue;
        }
        for j in 0..group_size {
            let num = x + j;
            if !map.contains_key(&num) {
                return false;
            }
            *map.get_mut(&num).unwrap() -= 1;
            if *map.get(&num).unwrap() == 0 {
                map.remove(&num);
            }
        }
    }
    true
}

fn main() {
    assert_eq!(is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 5], 3), true);
    assert_eq!(is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
}
