/*
 * @Date: 2023-06-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-14
 * @FilePath: /algorithm/rust/1375_num_times_all_blue/num_times_all_blue.rs
 */

pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut acc = 0;
    let mut sum = 0;
    for (index, item) in light.iter().enumerate() {
        sum = sum + index + 1;
        acc = acc + item;
        if sum as i32 == acc {
            res += 1;
        }
    }
    res
}

fn main() {
    {
        let flips = vec![3, 2, 4, 1, 5];
        let ans = 2;
        assert_eq!(num_times_all_blue(flips), ans);
    }

    {
        let flips = vec![4, 1, 2, 3];
        let ans = 1;
        assert_eq!(num_times_all_blue(flips), ans);
    }
}
