/*
 * @Date: 2022-07-08
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-08
 * @FilePath: /algorithm/1217_min_cost_to_move_chips/min_cost_to_move_chips.rs
 */

pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
    let (mut even, mut odd) = (0, 0);
    for pos in position {
        if pos % 2 != 0 {
            odd += 1;
        } else {
            even += 1;
        }
    }
    odd.min(even)
}

fn main() {
    assert_eq!(min_cost_to_move_chips(vec![1, 2, 3]), 1);
    assert_eq!(min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
    assert_eq!(min_cost_to_move_chips(vec![1, 10000000]), 1);
}
