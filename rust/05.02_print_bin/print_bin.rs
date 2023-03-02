/*
 * @Date: 2023-03-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-02
 * @FilePath: /algorithm/rust/05.02_print_bin/print_bin.rs
 */

pub fn print_bin(mut num: f64) -> String {
    let mut ans = String::from("0.");
    while num != 0.0 {
        num *= 2.0;
        if num >= 1.0 {
            ans.push('1');
            num -= 1.0;
        } else {
            ans.push('0');
        }

        if ans.len() > 32 {
            return "ERROR".to_string();
        }
    }
    ans
}

fn main() {
    {
        let num = 0.625;
        let ans = "0.101".to_string();
        assert_eq!(print_bin(num), ans);
    }

    {
        let num = 0.1;
        let ans = "ERROR".to_string();
        assert_eq!(print_bin(num), ans);
    }
}
