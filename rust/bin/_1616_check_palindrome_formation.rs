/*
 * @Date: 2023-03-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-18
 * @FilePath: /algorithm/rust/1616_check_palindrome_formation/check_palindrome_formation.rs
 */

pub fn check_palindrome_formation(a: String, b: String) -> bool {
    fn check_self_palindrome(s: &[char]) -> bool {
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right && s[left] == s[right] {
            left += 1;
            right -= 1;
        }
        left >= right
    }

    fn check_concatenation(a: &Vec<char>, b: &Vec<char>) -> bool {
        let (mut left, mut right) = (0, a.len() - 1);
        while left < right && a[left] == b[right] {
            left += 1;
            right -= 1;
        }
        if left >= right {
            return true;
        }
        check_self_palindrome(&a[left..right + 1]) || check_self_palindrome(&b[left..right + 1])
    }

    let (a, b) = (&a.chars().collect(), &b.chars().collect());
    check_concatenation(a, b) || check_concatenation(b, a)
}

fn main() {
    {
        let a = "x".to_string();
        let b = "y".to_string();
        let ans = true;
        assert_eq!(check_palindrome_formation(a, b), ans);
    }

    {
        let a = "abdef".to_string();
        let b = "fecab".to_string();
        let ans = true;
        assert_eq!(check_palindrome_formation(a, b), ans);
    }

    {
        let a = "ulacfd".to_string();
        let b = "jizalu".to_string();
        let ans = true;
        assert_eq!(check_palindrome_formation(a, b), ans);
    }
}
