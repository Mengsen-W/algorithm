/*
 * @Date: 2022-12-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-21
 * @FilePath: /algorithm/1753_maximum_score/maximum_score.rs
 */

pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
    let sum = a + b + c;
    let max_val = a.max(b.max(c));
    if sum - max_val < max_val {
        sum - max_val
    } else {
        sum / 2
    }
}

fn main() {
    {
        let (a, b, c, ans) = (2, 4, 6, 6);
        assert_eq!(maximum_score(a, b, c), ans);
    }

    {
        let (a, b, c, ans) = (4, 4, 6, 7);
        assert_eq!(maximum_score(a, b, c), ans);
    }

    {
        let (a, b, c, ans) = (1, 8, 8, 8);
        assert_eq!(maximum_score(a, b, c), ans);
    }
}
