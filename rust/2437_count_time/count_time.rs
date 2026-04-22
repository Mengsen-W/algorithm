/*
 * @Date: 2023-05-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-09
 * @FilePath: /algorithm/rust/2437_count_time/count_time.rs
 */

pub fn count_time(time: String) -> i32 {
    let t = time.chars().collect::<Vec<_>>();

    let hour = match (t[0], t[1]) {
        ('?', '?') => 24,
        ('?', '0'..='3') => 3,
        ('?', '4'..='9') => 2,
        ('0' | '1', '?') => 10,
        ('2', '?') => 4,
        _ => 1,
    };

    let minute = match (t[3], t[4]) {
        ('?', '?') => 60,
        ('?', _) => 6,
        (_, '?') => 10,
        _ => 1,
    };

    hour * minute
}

fn main() {
    {
        let time = "?5:00".to_string();
        let ans = 2;
        assert_eq!(count_time(time), ans);
    }

    {
        let time = "0?:0?".to_string();
        let ans = 100;
        assert_eq!(count_time(time), ans);
    }

    {
        let time = "??:??".to_string();
        let ans = 1440;
        assert_eq!(count_time(time), ans);
    }
}
