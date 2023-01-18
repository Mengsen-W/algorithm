/*
 * @Date: 2022-01-06 01:59:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-06 02:09:41
 */

pub fn simplify_path(path: String) -> String {
    let mut stack = Vec::new();
    for dir in path.split('/') {
        match dir {
            "." | "" => continue,
            ".." => {
                stack.pop();
            }
            _ => {
                stack.push(dir);
            }
        }
    }

    "/".to_string() + &stack.join("/")
}

fn main() {
    assert_eq!(simplify_path("/home/".to_string()), "/home".to_string());
    assert_eq!(simplify_path("/../".to_string()), "/".to_string());
    assert_eq!(
        simplify_path("/home//foo/".to_string()),
        "/home/foo".to_string()
    );
    assert_eq!(
        simplify_path("/a/./b/../../c/".to_string()),
        "/c".to_string()
    );
}
