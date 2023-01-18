/*
 * @Date: 2021-03-12 11:02:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-12 11:05:55
 * @FilePath: \algorithm\331_is_valid_serialization\debug.rs
 * @Description: file content
 */

fn main() {
    let nmus: Vec<i32> = vec![1, 2, 3, 4, 5];
    for mut i in (0..nmus.len() as i32).rev() {
        if i > 0 && i % 2 == 0 {
            i -= 1;
        }
        println!("{}", i);
    }
    // 改变 i 并不能降低迭代次数 和 c++ 不同
}
