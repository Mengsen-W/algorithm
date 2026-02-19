struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut ptr = 0;
        let mut last = 0;
        let mut ans = 0;

        while ptr < n {
            let c = bytes[ptr];
            let mut count = 0;
            while ptr < n && bytes[ptr] == c {
                ptr += 1;
                count += 1;
            }
            ans += count.min(last);
            last = count;
        }

        ans
    }
}

fn main() {
    let tests = vec![("00110011", 6), ("10101", 4)];

    for (s, ans) in tests {
        assert_eq!(Solution::count_binary_substrings(s.to_string()), ans);
    }
}
