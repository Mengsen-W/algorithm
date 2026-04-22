struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut diff = vec![0; 26];
        for c in word2.chars() {
            diff[(c as u8 - b'a') as usize] -= 1;
        }

        let mut res = 0;
        let mut cnt = diff.iter().filter(|&&c| c < 0).count();

        let mut update = |c: usize, add: i32, cnt: &mut usize| {
            diff[c] += add;
            if add == 1 && diff[c] == 0 {
                // 表明 diff[c] 由 -1 变为 0
                *cnt -= 1;
            } else if add == -1 && diff[c] == -1 {
                // 表明 diff[c] 由 0 变为 -1
                *cnt += 1;
            }
        };

        let (mut l, mut r) = (0, 0);
        let n = word1.len();
        let bytes = word1.as_bytes();

        while l < n {
            while r < n && cnt > 0 {
                update((bytes[r] - b'a') as usize, 1, &mut cnt);
                r += 1;
            }
            if cnt == 0 {
                res += (n - r) as i64 + 1;
            }
            update((bytes[l] - b'a') as usize, -1, &mut cnt);
            l += 1;
        }

        res
    }
}

fn main() {
    let tests = vec![
        ("bcca", "abc", 1),
        ("abcabc", "abc", 10),
        ("abcabc", "aaabc", 0),
    ];

    for (word1, word2, ans) in tests {
        assert_eq!(
            Solution::valid_substring_count(word1.to_string(), word2.to_string()),
            ans
        );
    }
}
