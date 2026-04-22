/*
 * @Date: 2023-04-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-12
 * @FilePath: /algorithm/rust/1147_longest_decomposition/longest_decomposition.rs
 */

pub fn longest_decomposition(text: String) -> i32 {
    let n = text.len();
    let letters = text.chars().collect::<Vec<char>>();
    let mut counter = 0;
    let (mut prefix, mut suffix) = (String::new(), String::new());

    for i in 0..n / 2 {
        prefix.push(letters[i]);
        suffix.push(letters[n - i - 1]);
        if prefix == suffix.chars().rev().collect::<String>() {
            counter += 2;
            prefix.clear();
            suffix.clear();
        }
    }

    if n % 2 == 1 || prefix.len() > 0 {
        counter + 1
    } else {
        counter
    }
}

fn main() {
    {
        let text = String::from("ghiabcdefhelloadamhelloabcdefghi");
        let ans = 7;
        assert_eq!(longest_decomposition(text), ans);
    }

    {
        let text = String::from("merchant");
        let ans = 1;
        assert_eq!(longest_decomposition(text), ans);
    }

    {
        let text = String::from("antaprezatepzapreanta");
        let ans = 11;
        assert_eq!(longest_decomposition(text), ans);
    }
}
