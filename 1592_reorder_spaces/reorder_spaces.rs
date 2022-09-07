/*
 * @Date: 2022-09-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-07
 * @FilePath: /algorithm/1592_reorder_spaces/reorder_spaces.rs
 */

pub fn reorder_spaces(text: String) -> String {
    let arr = text
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let (word_cnt, space_cnt) = (
        arr.len(),
        text.as_bytes().iter().filter(|x| **x == ' ' as u8).count(),
    );
    let repeat = if word_cnt <= 1 {
        0
    } else {
        space_cnt / (word_cnt - 1)
    };
    format!(
        "{}{}",
        arr.join(" ".repeat(repeat).as_str()),
        " ".repeat(space_cnt - repeat * (word_cnt - 1))
    )
}

fn main() {
    {
        let text = String::from("  this   is  a sentence ");
        let ans = String::from("this   is   a   sentence");
        assert_eq!(reorder_spaces(text), ans);
    }

    {
        let text = String::from(" practice   makes   perfect");
        let ans = String::from("practice   makes   perfect ");
        assert_eq!(reorder_spaces(text), ans);
    }

    {
        let text = String::from("hello   world");
        let ans = String::from("hello   world");
        assert_eq!(reorder_spaces(text), ans);
    }

    {
        let text = String::from("  walks  udp package   into  bar a");
        let ans = String::from("walks  udp  package  into  bar  a ");
        assert_eq!(reorder_spaces(text), ans);
    }

    {
        let text = String::from("a");
        let ans = String::from("a");
        assert_eq!(reorder_spaces(text), ans);
    }
}
