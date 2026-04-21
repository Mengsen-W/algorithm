/*
 * @Date: 2022-12-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-25
 * @FilePath: /algorithm/1739_minimum_boxes/minimum_boxes.rs
 */

pub fn minimum_boxes(n: i32) -> i32 {
    let mut n = n as f64;
    fn g(x: i64) -> f64 {
        (x * (x + 1) * (x + 2) / 6) as f64
    }
    let mut i = ((6.0 * n).powf(1.0 / 3.0)) as i64;
    if g(i) < n {
        i += 1;
    }
    n -= g(i - 1);
    let j = ((((8.0 * n + 1.0).sqrt() - 1.0) / 2.0).ceil()) as i64;
    ((i - 1) * i / 2 + j) as i32
}

fn main() {
    {
        let n = 3;
        let ans = 3;
        assert_eq!(minimum_boxes(n), ans);
    }

    {
        let n = 4;
        let ans = 3;
        assert_eq!(minimum_boxes(n), ans);
    }

    {
        let n = 10;
        let ans = 6;
        assert_eq!(minimum_boxes(n), ans);
    }

    let n = 424859355;
    minimum_boxes(n);
}
