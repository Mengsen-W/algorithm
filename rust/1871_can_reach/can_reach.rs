struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let n = s.len();
        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;
        let mut f = vec![0; n];
        let mut pre = vec![0; n];
        f[0] = 1;
        // 由于我们从 i=minJump 开始动态规划，因此需要将 [0,minJump) 这部分的前缀和预处理出来
        for i in 0..min_jump {
            pre[i] = 1;
        }
        let s_chars: Vec<char> = s.chars().collect();
        for i in min_jump..n {
            let left = i as i32 - max_jump as i32;
            let right = i - min_jump;
            if s_chars[i] == '0' {
                let total = if left <= 0 {
                    pre[right]
                } else {
                    pre[right] - pre[left as usize - 1]
                };
                f[i] = if total != 0 { 1 } else { 0 };
            }
            pre[i] = pre[i - 1] + f[i];
        }
        f[n - 1] == 1
    }
}

fn main() {
    let tests = vec![("011010", 2, 3, true), ("01101110", 2, 3, false)];

    for (s, min_jump, max_jump, expected) in tests {
        assert_eq!(
            Solution::can_reach(s.to_string(), min_jump, max_jump),
            expected
        );
    }
}
