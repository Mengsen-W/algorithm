/*
 * @Date: 2022-09-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-13
 * @FilePath: /algorithm/670_maximum_swap/maximum_swap.rs
 */

pub fn maximum_swap(num: i32) -> i32 {
    let mut s: Vec<_> = format!("{}", num).to_string().bytes().collect();
    let (mut max, mut max_id) = (b'0' - 1, s.len());
    let (mut id1, mut id2) = (s.len(), s.len());
    for i in (0..s.len()).rev() {
        if s[i] > max {
            max = s[i];
            max_id = i;
        } else if s[i] < max {
            id1 = max_id;
            id2 = i;
        }
    }

    if id1 != s.len() && id2 != s.len() {
        s.swap(id1, id2);
        String::from_utf8(s).unwrap().parse().unwrap()
    } else {
        num
    }
}

fn main() {
    assert_eq!(maximum_swap(2736), 7236);
    assert_eq!(maximum_swap(9973), 9973);
}
