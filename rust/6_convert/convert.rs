/*
 * @Date: 2022-03-01 00:06:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-01 01:45:55
 * @FilePath: /algorithm/6_convert/convert.rs
 */

pub fn convert1(s: String, num_rows: i32) -> String {
    let (n, r) = (s.len(), num_rows as usize);
    if r == 1 || r >= n {
        return s;
    }
    s.chars()
        .collect::<Vec<char>>()
        .iter()
        .enumerate()
        .fold(
            (
                vec![vec!['0'; (n + (r * 2 - 2) - 1) / (r * 2 - 2) * (r - 1)]; r],
                0,
                0,
            ),
            |(mut mat, mut x, mut y), (i, ch)| {
                mat[x][y] = *ch;
                if i % (r * 2 - 2) < r - 1 {
                    x += 1;
                } else {
                    x -= 1;
                    y += 1;
                }
                (mat, x, y)
            },
        )
        .0
        .iter()
        .map(|v| v.iter().filter(|x| **x != '0').collect::<String>())
        .collect::<Vec<String>>()
        .join("")
}

pub fn convert2(s: String, num_rows: i32) -> String {
    let (n, r) = (s.len(), num_rows as usize);
    if r == 1 || r >= n {
        return s;
    }

    s.chars()
        .collect::<Vec<char>>()
        .iter()
        .enumerate()
        .fold((vec![String::new(); r], 0), |(mut mat, mut x), (i, ch)| {
            mat.get_mut(x).unwrap_or(&mut String::new()).push(*ch);
            if i % (r * 2 - 2) < r - 1 {
                x += 1;
            } else {
                x -= 1;
            }
            (mat, x)
        })
        .0
        .join("")
}

pub fn convert3(s: String, num_rows: i32) -> String {
    let (n, r) = (s.len(), num_rows as usize);
    if r == 1 || r >= n {
        return s;
    }

    let s = s.chars().collect::<Vec<char>>();
    let mut ans = String::new();
    for i in 0..r {
        for j in (0..n - i).step_by(r * 2 - 2) {
            ans.push(s[j + i]);
            if 0 < i && i < r - 1 && j + (r * 2 - 2) - i < n {
                ans.push(s[j + (r * 2 - 2) - i]);
            }
        }
    }
    ans
}

pub fn convert4(s: String, num_rows: i32) -> String {
    // 为什么输入类型总是i32
    let num_rows = num_rows as usize;
    // 每行一个String
    let mut rows = vec![String::new(); num_rows];
    // z字形往复的迭代器，01232101232......
    let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
    // 按迭代器给出的下标访问对应行，推入字符
    iter.zip(s.chars()).for_each(|(i, c)| rows[i].push(c));
    // collect连接每行
    rows.into_iter().collect()
}

fn main() {
    assert_eq!(
        convert1("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
        convert2("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
        convert3("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );

    assert_eq!(
        convert1("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI".to_string()
    );
    assert_eq!(
        convert2("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI".to_string()
    );
    assert_eq!(
        convert3("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI".to_string()
    );

    assert_eq!(convert1("A".to_string(), 1), "A".to_string());
    assert_eq!(convert2("A".to_string(), 1), "A".to_string());
    assert_eq!(convert3("A".to_string(), 1), "A".to_string());
}
