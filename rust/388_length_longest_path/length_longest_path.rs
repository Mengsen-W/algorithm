/*
 * @Date: 2022-04-20 09:17:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-20 09:32:07
 * @FilePath: /algorithm/388_length_longest_path/length_longest_path.rs
 */

pub fn length_longest_path(input: String) -> i32 {
    let mut stk: Vec<&str> = vec![];
    let mut sum = 0;
    let mut ans = 0;
    for row in input.split('\n') {
        let name = row.trim_start_matches(|c| c == '\t');
        let depth = row.len() - name.len();
        while depth < stk.len() {
            sum -= stk.pop().unwrap().len();
        }
        sum += name.len();
        if name.contains('.') && sum + stk.len() > ans {
            ans = sum + stk.len();
        }
        stk.push(name);
    }
    ans as i32
}

fn main() {
    assert_eq!(
        length_longest_path(String::from("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext")),
        20
    );
    assert_eq!(length_longest_path(String::from("a")), 0);
    assert_eq!(length_longest_path(String::from("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext")) , 32);
}
