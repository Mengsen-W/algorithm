/*
 * @Date: 2022-10-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-02
 * @FilePath: /algorithm/777_can_transform/can_transform.rs
 */

pub fn can_transform(start: String, end: String) -> bool {
    let m = start.len();
    let n = end.len();
    if m != n {
        return false;
    }
    let s = start.as_str().chars();
    let mut e = end.as_str().chars();
    let mut j = 0;
    for (i, y) in s.enumerate() {
        if y == 'L' {
            if j == m {
                return false;
            }
            loop {
                j += 1;
                match e.next() {
                    Some(x) => {
                        if x == 'L' {
                            break;
                        }
                        if x == 'R' {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
            if i + 1 < j {
                return false;
            }
        } else if y == 'R' {
            if j == m {
                return false;
            }
            loop {
                j += 1;
                match e.next() {
                    Some(x) => {
                        if x == 'R' {
                            break;
                        }
                        if x == 'L' {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
            if i + 1 > j {
                return false;
            }
        }
    }
    loop {
        match e.next() {
            Some(x) => {
                if x != 'X' {
                    return false;
                }
            }
            None => break,
        }
    }
    true
}

fn main() {
    assert!(can_transform(
        String::from("RXXLRXRXL"),
        String::from("XRLXXRRLX")
    ));
}
