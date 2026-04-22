/*
 * @Date: 2023-04-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-09
 * @FilePath: /algorithm/rust/2399_check_distances/check_distances.rs
 */

pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
    let mut cache = vec![-1; 26];
    for (i, ch) in s.bytes().enumerate() {
        let j = (ch - b'a') as usize;
        if cache[j] != -1 && i as i32 - cache[j] - 1 != distance[j] {
            return false;
        }
        cache[j] = i as i32;
    }
    true
}

fn main() {
    {
        let s = String::from("abaccb");
        let distance = vec![
            1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let ans = true;
        assert_eq!(check_distances(s, distance), ans);
    }

    {
        let s = String::from("aa");
        let distance = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let ans = false;
        assert_eq!(check_distances(s, distance), ans);
    }
}
