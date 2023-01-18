/*
 * @Date: 2021-09-01 13:49:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-01 15:20:53
 */

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let n = version1.len();
        let m = version2.len();
        let version1: Vec<char> = version1.chars().collect();
        let version2: Vec<char> = version2.chars().collect();
        let (mut i, mut j) = (0, 0);
        while i < n || j < m {
            let mut x = 0;
            while i < n && version1[i] != '.' {
                x = x * 10 + (version1[i] as i32 - '0' as i32);
                i += 1;
            }
            i += 1;
            let mut y = 0;
            while j < m && version2[j] != '.' {
                y = y * 10 + (version2[j] as i32 - '0' as i32);
                j += 1;
            }
            j += 1;
            if x != y {
                match x > y {
                    true => return 1,
                    false => return -1,
                }
            }
        }
        return 0;
    }
}

fn main() {
    {
        let version1 = "1.01".to_string();
        let version2 = "1.001".to_string();
        assert_eq!(Solution::compare_version(version1, version2), 0);
    }
    {
        let version1 = "1.0".to_string();
        let version2 = "1.0.0".to_string();
        assert_eq!(Solution::compare_version(version1, version2), 0);
    }
    {
        let version1 = "0.1".to_string();
        let version2 = "1.1".to_string();
        assert_eq!(Solution::compare_version(version1, version2), -1);
    }
    {
        let version1 = "1.0.1".to_string();
        let version2 = "1".to_string();
        assert_eq!(Solution::compare_version(version1, version2), 1);
    }
    {
        let version1 = "7.5.2.4".to_string();
        let version2 = "7.5.3".to_string();
        assert_eq!(Solution::compare_version(version1, version2), -1);
    }
}
