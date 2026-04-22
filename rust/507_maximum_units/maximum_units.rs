/*
 * @Date: 2022-11-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-15
 * @FilePath: /algorithm/507_maximum_units/maximum_units.rs
 */

pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut box_types = box_types;
    let mut truck_size = truck_size;
    box_types.sort_unstable_by(|a, b| b[1].cmp(&a[1]));
    let mut res = 0;
    for box_type in box_types {
        let number_of_boxes = box_type[0];
        let number_of_units_per_box = box_type[1];
        if number_of_boxes < truck_size {
            res += number_of_boxes * number_of_units_per_box;
            truck_size -= number_of_boxes;
        } else {
            res += truck_size * number_of_units_per_box;
            break;
        }
    }
    res
}

fn main() {
    {
        let box_types = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
        let truck_size = 4;
        let ans = 8;
        assert_eq!(maximum_units(box_types, truck_size), ans);
    }
    {
        let box_types = vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]];
        let truck_size = 10;
        let ans = 91;
        assert_eq!(maximum_units(box_types, truck_size), ans);
    }
}
