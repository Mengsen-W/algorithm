struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let (mut leftb, mut righta) = (0, 0);
        for i in 0..s.len() {
            if s[i] == 'a' {
                righta += 1;
            }
        }
        let mut res = righta;
        for i in 0..s.len() {
            let c = s[i];
            if c == 'a' {
                righta -= 1;
            } else {
                leftb += 1;
            }
            res = res.min(leftb + righta);
        }
        res
    }
}

fn main() {
    let tests = vec![("aababbab", 2), ("bbaaaaabb", 2)];

    for (s, expected) in tests {
        let result = Solution::minimum_deletions(s.to_string());
    }
}
