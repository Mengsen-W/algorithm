/*
 * @Date: 2023-05-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-06
 * @FilePath: /algorithm/rust/1419_min_number_of_frogs/min_number_of_frogs.rs
 */

pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
    let (mut c, mut r, mut o, mut a, mut k) = (0, 0, 0, 0, 0);
    let (mut n, mut m) = (0, 0);

    for b in croak_of_frogs.bytes() {
        match b {
            b'c' => {
                c += 1;
                m += 1;
            }
            b'r' => r += 1,
            b'o' => o += 1,
            b'a' => a += 1,
            b'k' => {
                k += 1;
                m -= 1;
            }
            _ => {}
        };

        if c < r || r < o || o < a || a < k {
            return -1;
        }

        n = n.max(m);
    }

    if m == 0 {
        n
    } else {
        -1
    }
}

fn main() {
    {
        let croak_of_frogs = "croakcroak".to_string();
        let ans = 1;
        assert_eq!(min_number_of_frogs(croak_of_frogs), ans);
    }

    {
        let croak_of_frogs = "crcoakroak".to_string();
        let ans = 2;
        assert_eq!(min_number_of_frogs(croak_of_frogs), ans);
    }

    {
        let croak_of_frogs = "croakcrook".to_string();
        let ans = -1;
        assert_eq!(min_number_of_frogs(croak_of_frogs), ans);
    }
}
