/*
 * @Date: 2023-02-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-14
 * @FilePath: /algorithm/rust/1124_longest_wpi/longest_wpi.rs
 */

pub fn longest_wpi(hours: Vec<i32>) -> i32 {
    let n = hours.len() as i32;
    let mut ump: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let (mut s, mut res): (i32, i32) = (0, 0);
    for i in 0..n {
        if hours[i as usize] > 8 {
            s += 1;
        } else {
            s -= 1;
        }
        if s > 0 {
            res = res.max(i + 1);
        } else {
            if let Some(v) = ump.get(&(s - 1)) {
                res = res.max(i - *v);
            }
        }
        if ump.get(&s).is_none() {
            ump.insert(s, i);
        }
    }
    res
}

fn main() {
    {
        let hours = vec![9, 9, 6, 0, 6, 6, 9];
        let ans = 3;
        assert_eq!(longest_wpi(hours), ans);
    }

    {
        let hours = vec![6, 6, 6];
        let ans = 0;
        assert_eq!(longest_wpi(hours), ans);
    }
}
