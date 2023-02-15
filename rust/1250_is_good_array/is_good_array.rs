/*
 * @Date: 2023-02-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-15
 * @FilePath: /algorithm/rust/1250_is_good_array/is_good_array.rs
 */

pub fn is_good_array(nums: Vec<i32>) -> bool {
    nums.into_iter()
        .reduce(|mut b, mut a| {
            while a != 0 {
                let tmp = b;
                b = a;
                a = tmp.rem_euclid(a);
            }
            b
        })
        .unwrap()
        == 1
}

fn main() {
    {
        let nums = vec![12, 5, 7, 23];
        assert_eq!(is_good_array(nums), true);
    }

    {
        let nums = vec![29, 6, 10];
        assert_eq!(is_good_array(nums), true);
    }

    {
        let nums = vec![3, 6];
        assert_eq!(is_good_array(nums), false);
    }
}
