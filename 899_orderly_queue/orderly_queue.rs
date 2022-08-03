/*
 * @Date: 2022-08-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-03
 * @FilePath: /algorithm/899_orderly_queue/orderly_queue.rs
 */

pub fn orderly_queue(s: String, k: i32) -> String {
    return if k > 1 {
        let mut arr = s.chars().collect::<Vec<_>>();
        arr.sort();
        arr.iter().collect::<_>()
    } else {
        let tmp = s.chars().chain(s.chars()).collect::<Vec<_>>();
        let mut v = tmp.windows(s.len()).collect::<Vec<_>>();
        v.sort();
        v[0].iter().copied().collect::<_>()
    };
}

fn main() {
    assert_eq!(orderly_queue(String::from("cba"), 1), String::from("acb"));
    assert_eq!(
        orderly_queue(String::from("baaca"), 3),
        String::from("aaabc")
    );
}
