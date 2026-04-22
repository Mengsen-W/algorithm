/*
 * @Date: 2022-08-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-24
 * @FilePath: /algorithm/1460_can_be_equal/can_be_equal.rs
 */

pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
    target.sort();
    arr.sort();
    target == arr
}

fn main() {
    {
        let target = vec![1, 2, 3, 4];
        let arr = vec![2, 4, 1, 3];
        assert!(can_be_equal(target, arr));
    }
    {
        let target = vec![7];
        let arr = vec![7];
        assert!(can_be_equal(target, arr));
    }

    {
        let target = vec![3, 7, 9];
        let arr = vec![3, 7, 11];
        assert!(!can_be_equal(target, arr));
    }
}
