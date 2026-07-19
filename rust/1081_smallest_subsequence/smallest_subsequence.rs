struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut vis = [0; 26];
        let mut num = [0; 26];

        for ch in s.chars() {
            num[(ch as u8 - b'a') as usize] += 1;
        }
        let mut stk: Vec<char> = Vec::new();

        for ch in s.chars() {
            let idx = (ch as u8 - b'a') as usize;
            if vis[idx] == 0 {
                while let Some(&top) = stk.last() {
                    let top_idx = (top as u8 - b'a') as usize;
                    if top > ch && num[top_idx] > 0 {
                        vis[top_idx] = 0;
                        stk.pop();
                    } else {
                        break;
                    }
                }
                vis[idx] = 1;
                stk.push(ch);
            }
            num[idx] -= 1;
        }

        stk.into_iter().collect()
    }
}

fn main() {
    let tests = vec![("bcabc", "abc"), ("cbacdcbc", "acdb")];

    for (s, ans) in tests {
        assert_eq!(Solution::smallest_subsequence(s.to_string()), ans);
    }
}
