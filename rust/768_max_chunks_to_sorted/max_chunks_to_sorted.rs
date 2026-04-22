/*
 * @Date: 2022-08-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-15
 * @FilePath: /algorithm/768_max_chunks_to_sorted/max_chunks_to_sorted.rs
 */

pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    use std::collections::VecDeque;

    let mut st = VecDeque::new();
    st.push_back(-1);

    for n in arr.into_iter() {
        let pre = n.max(*st.back().unwrap());
        while let Some(top) = st.pop_back() {
            if n >= top {
                st.push_back(top);
                st.push_back(pre);
                break;
            }
        }
    }

    st.len() as i32 - 1
}

fn main() {
    assert_eq!(max_chunks_to_sorted(vec![5, 4, 3, 2, 1]), 1);
    assert_eq!(max_chunks_to_sorted(vec![2, 1, 3, 4, 4]), 4);
}
