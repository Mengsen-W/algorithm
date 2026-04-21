/*
 * @Date: 2022-09-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-09
 * @FilePath: /algorithm/1598_min_operations/min_operations.rs
 */

pub fn min_operations(logs: Vec<String>) -> i32 {
    logs.iter().fold(0, |mut ans, log| {
        match log.as_str() {
            "../" => {
                if ans > 0 {
                    ans -= 1
                }
            }
            "./" => {}
            _ => ans += 1,
        };
        ans
    })
}

fn main() {
    {
        let logs = vec!["d1/", "d2/", "../", "d21/", "./"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let ans = 2;
        assert_eq!(min_operations(logs), ans);
    }

    {
        let logs = vec!["d1/", "d2/", "./", "d3/", "../", "d31/"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let ans = 3;
        assert_eq!(min_operations(logs), ans);
    }

    {
        let logs = vec!["d1/", "../", "../", "../"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let ans = 0;
        assert_eq!(min_operations(logs), ans);
    }
}
