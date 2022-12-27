/*
 * @Date: 2022-12-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-27
 * @FilePath: /algorithm/2027_minimum_moves/minimum_moves.rs
 */

pub fn minimum_moves(s: String) -> i32 {
    s.bytes()
        .fold((0, 0), |s, x| {
            if x == b'X' && s.1 <= 0 {
                (s.0 + 1, 2)
            } else {
                (s.0, s.1 - 1)
            }
        })
        .0
}

fn main() {
    {
        let s = String::from("XXX");
        let ans = 1;
        assert_eq!(minimum_moves(s), ans);
    }

    {
        let s = String::from("XXOX");
        let ans = 2;
        assert_eq!(minimum_moves(s), ans);
    }

    {
        let s = String::from("OOOO");
        let ans = 0;
        assert_eq!(minimum_moves(s), ans);
    }
}
