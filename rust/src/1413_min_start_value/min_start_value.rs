/*
 * @Date: 2022-08-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-09
 * @FilePath: /algorithm/1413_min_start_value/min_start_value.rs
 */

pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut start_value = 1;
    'outer: loop {
        let mut tmp_sum = start_value;
        for num in &nums {
            tmp_sum += num;
            if tmp_sum < 1 {
                start_value += 1;
                continue 'outer;
            }
        }
        break;
    }
    start_value
}

fn main() {
    assert_eq!(min_start_value(vec![-3, 2, -3, 4, 2]), 5);
    assert_eq!(min_start_value(vec![1, 2]), 1);
    assert_eq!(min_start_value(vec![1, -2, -3]), 5);
}
