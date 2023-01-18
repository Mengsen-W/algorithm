/*
 * @Date: 2022-07-04
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-04
 * @FilePath: /algorithm/1200_minimum_abs_difference/minimum_abs_difference.rs
 */

pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    arr.sort();
    let min = arr
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .min()
        .unwrap();
    arr.windows(2)
        .filter_map(|slice| {
            if slice[1] - slice[0] == min {
                Some(slice.to_vec())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn main() {
    {
        let arr = vec![4, 2, 1, 3];
        let ans = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(minimum_abs_difference(arr), ans);
    }

    {
        let arr = vec![1, 3, 6, 10, 15];
        let ans = vec![vec![1, 3]];
        assert_eq!(minimum_abs_difference(arr), ans);
    }

    {
        let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let ans = vec![vec![-14, -10], vec![19, 23], vec![23, 27]];
        assert_eq!(minimum_abs_difference(arr), ans);
    }
}
