/*
 * @Date: 2021-07-29 09:47:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-29 12:01:47
 */

fn path_in_zig_zag_tree(mut label: i32) -> Vec<i32> {
    let mut ans = vec![];
    while label >= 1 {
        ans.push(label);
        label >>= 1;
    }
    ans.reverse();
    let mut i = ans.len() as i32 - 2;
    while i > 0 {
        ans[i as usize] = 2_i32.pow(i as u32) + 2_i32.pow(i as u32 + 1) - 1 - ans[i as usize];
        i -= 2;
    }
    ans
}

fn main() {
    {
        let label = 14;
        let ans = vec![1, 3, 4, 14];
        assert_eq!(path_in_zig_zag_tree(label), ans);
    }
    {
        let label = 26;
        let ans = vec![1, 2, 6, 10, 26];
        assert_eq!(path_in_zig_zag_tree(label), ans);
    }
}
