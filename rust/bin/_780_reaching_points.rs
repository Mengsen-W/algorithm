/*
 * @Date: 2022-04-09 07:35:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-09 07:45:46
 * @FilePath: /algorithm/780_reaching_points/reaching_points.rs
 */

pub fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
    while tx > sx && ty > sy && tx != ty {
        if tx > ty {
            tx %= ty;
        } else {
            ty %= tx;
        }
    }
    if tx == sx && ty == sy {
        return true;
    } else if tx == sx {
        return ty > sy && (ty - sy) % tx == 0;
    } else if ty == sy {
        return tx > sx && (tx - sx) % ty == 0;
    } else {
        return false;
    }
}

fn main() {
    assert_eq!(reaching_points(1, 1, 3, 5), true);
    assert_eq!(reaching_points(1, 1, 2, 2), false);
    assert_eq!(reaching_points(1, 1, 1, 1), true);
}
