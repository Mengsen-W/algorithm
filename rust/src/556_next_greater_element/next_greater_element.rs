/*
 * @Date: 2022-07-04
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-04
 * @FilePath: /algorithm/556_next_greater_element/next_greater_element.rs
 */

pub fn next_greater_element(n: i32) -> i32 {
    let mut ch_arr = n.to_string().chars().collect::<Vec<char>>();
    let (mut i, mut j): (i32, usize) = (ch_arr.len() as i32 - 2, ch_arr.len() - 1);
    while i >= 0 as i32 && ch_arr[i as usize] >= ch_arr[i as usize + 1] {
        i -= 1;
    }
    if i == -1 {
        return -1;
    }
    while j < ch_arr.len() && ch_arr[i as usize] >= ch_arr[j] {
        j -= 1;
    }
    ch_arr.swap(i as usize, j);
    let (mut prefix, mut tmp) = (
        ch_arr[..i as usize + 1].to_vec(),
        ch_arr[i as usize + 1..].to_vec(),
    );
    tmp.sort();
    prefix.append(&mut tmp);
    return if let Ok(num) = prefix.iter().map(|c| *c).collect::<String>().parse::<i32>() {
        num
    } else {
        -1
    };
}

fn main() {
    assert_eq!(next_greater_element(12), 21);
    assert_eq!(next_greater_element(21), -1);
}
