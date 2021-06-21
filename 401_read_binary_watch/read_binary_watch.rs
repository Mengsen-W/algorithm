/*
 * @Date: 2021-06-21 08:30:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-21 09:01:11
 */

fn read_binary_watch_enum_hourminutes(turned_on: i32) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    for h in 0 as u8..12 {
        for m in 0 as u8..60 {
            if h.count_ones() + m.count_ones() == turned_on as u32 {
                let mut minutes = String::new();
                if m < 10 {
                    minutes = 0.to_string();
                }
                ans.push(format!("{}:{}{}", h, minutes, m));
            }
        }
    }
    ans
}

fn read_binary_watch_enum_binary(turned_on: i32) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    for i in 0..1024 as i32 {
        let h = i >> 6;
        let m = i & 63;
        if h < 12 && m < 60 && i.count_ones() == turned_on as u32 {
            let mut minutes = String::new();
            if m < 10 {
                minutes = 0.to_string();
            }
            ans.push(format!("{}:{}{}", h, minutes, m));
        }
    }
    ans
}

fn main() {
    assert_eq!(
        read_binary_watch_enum_hourminutes(1),
        read_binary_watch_enum_binary(1)
    );
    assert_eq!(
        read_binary_watch_enum_hourminutes(9),
        read_binary_watch_enum_binary(9)
    );
}
