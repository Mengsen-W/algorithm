struct Solution;

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();
        let mut ans = vec![folder[0].clone()];
        for s in folder.into_iter().skip(1) {
            let last = ans.last().unwrap();
            if !s.starts_with(last) || s.as_bytes()[last.len()] != b'/' {
                ans.push(s);
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"],
            vec!["/a", "/c/d", "/c/f"],
        ),
        (vec!["/a", "/a/b/c", "/a/b/d"], vec!["/a"]),
        (
            vec!["/a/b/c", "/a/b/ca", "/a/b/d"],
            vec!["/a/b/c", "/a/b/ca", "/a/b/d"],
        ),
    ];

    for (folder, expected) in tests {
        assert_eq!(
            Solution::remove_subfolders(folder.iter().map(|s| s.to_string()).collect()),
            expected
        );
    }
}
