struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let n = s.len();
        let s: Vec<u8> = s.bytes().collect();
        let mut res = 0;
        for i in 0..n {
            let mut count = [0, 0];
            for j in i..n {
                count[s[j] as usize - b'0' as usize] += 1;
                if count[0] > k && count[1] > k {
                    break;
                }
                res += 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![("10101", 1, 12), ("1010101", 2, 25), ("11111", 1, 15)];

    for (s, k, ans) in tests {
        assert_eq!(
            Solution::count_k_constraint_substrings(s.to_string(), k),
            ans
        );
    }
}
