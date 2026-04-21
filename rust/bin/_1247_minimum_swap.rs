/*
 * @Date: 2023-02-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-25
 * @FilePath: /algorithm/rust/1247_minimum_swap/minimum_swap.rs
 */

pub fn minimum_swap(s1: String, s2: String) -> i32 {
    let (xy, yx) = s1
        .chars()
        .zip(s2.chars())
        .fold((0, 0), |(xy, yx), (c1, c2)| {
            if c1 == c2 {
                (xy, yx)
            } else if c1 == 'x' {
                (xy + 1, yx)
            } else {
                (xy, yx + 1)
            }
        });
    if (xy + yx) & 1 == 1 {
        return -1;
    }
    xy / 2 + yx / 2 + (xy & 1) * 2
}

fn main() {
    {
        let s1 = "xx".to_string();
        let s2 = "yy".to_string();
        let ans = 1;
        assert_eq!(minimum_swap(s1, s2), ans);
    }

    {
        let s1 = "xy".to_string();
        let s2 = "yx".to_string();
        let ans = 2;
        assert_eq!(minimum_swap(s1, s2), ans);
    }

    {
        let s1 = "xx".to_string();
        let s2 = "xy".to_string();
        let ans = -1;
        assert_eq!(minimum_swap(s1, s2), ans);
    }

    {
        let s1 = "xxyyxyxyxx".to_string();
        let s2 = "xyyxyxxxyx".to_string();
        let ans = 4;
        assert_eq!(minimum_swap(s1, s2), ans);
    }
}
