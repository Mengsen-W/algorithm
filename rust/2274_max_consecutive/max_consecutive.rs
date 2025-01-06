struct Solution;

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let mut special = special;
        special.push(bottom - 1);
        special.push(top + 1);
        special.sort();
        let mut ans = 0;
        for i in 0..special.len() - 1 {
            ans = ans.max(special[i + 1] - special[i] - 1);
        }
        ans
    }
}

fn main() {
    let tests = vec![(2, 9, vec![4, 6], 3), (6, 8, vec![7, 6, 8], 0)];

    for (bottom, top, special, ans) in tests {
        assert_eq!(Solution::max_consecutive(bottom, top, special), ans);
    }
}
