/*
 * @Date: 2022-08-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-26
 * @FilePath: /algorithm/1464_max_product/max_product.rs
 */

pub fn max_product(nums: Vec<i32>) -> i32 {
    let (mut first, mut second) = (0, 0);
    nums.iter().for_each(|&x| {
        if x > first {
            second = first;
            first = x;
        } else if x > second {
            second = x;
        }
    });
    (first - 1) * (second - 1)
}

fn main() {
    {
        let nums = vec![3, 4, 5, 2];
        let ans = 12;
        assert_eq!(max_product(nums), ans);
    }
    {
        let nums = vec![1, 5, 4, 5];
        let ans = 16;
        assert_eq!(max_product(nums), ans);
    }
    {
        let nums = vec![3, 7];
        let ans = 12;
        assert_eq!(max_product(nums), ans);
    }
}
