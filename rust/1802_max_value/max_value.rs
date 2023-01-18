/*
 * @Date: 2023-01-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-04
 * @FilePath: /algorithm/1802_max_value/max_value.rs
 */

pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
    let max_sum = max_sum as f64;
    let mut left = index as f64;
    let mut right = n as f64 - index as f64 - 1.0;

    if left > right {
        let tmp = left;
        left = right;
        right = tmp;
    }

    let mut upper = ((left + 1.0) * (left + 1.0) - 3.0 * (left + 1.0)) / 2.0
        + left
        + 1.0
        + (left + 1.0)
        + ((left + 1.0) * (left + 1.0) - 3.0 * (left + 1.0)) / 2.0
        + right
        + 1.0;

    if upper >= max_sum {
        let a = 1.0;
        let b = -2.0;
        let c = left + right + 2.0 - max_sum;
        return ((-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)).floor() as i32;
    }

    upper = ((right + 1.0) * 2.0 - left - 1.0) * left / 2.0
        + (right + 1.0)
        + ((right + 1.0) * (right + 1.0) - (right + 1.0) * 3.0) / 2.0
        + right
        + 1.0;

    if upper >= max_sum {
        let a = 1.0 / 2.0;
        let b = left + 1.0 - 3.0 / 2.0;
        let c = right + 1.0 + (-left - 1.0) * left / 2.0 - max_sum;
        return ((-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)).floor() as i32;
    } else {
        let a = left + right + 1.0;
        let b = (-left * left - left - right * right - right) / 2.0 - max_sum;
        return (-b / a).floor() as i32;
    }
}

fn main() {
    {
        let (n, index, max_sum) = (4, 2, 6);
        let ans = 2;
        assert_eq!(max_value(n, index, max_sum), ans);
    }

    {
        let (n, index, max_sum) = (6, 1, 10);
        let ans = 3;
        assert_eq!(max_value(n, index, max_sum), ans);
    }
}
