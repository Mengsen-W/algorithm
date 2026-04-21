/*
 * @Date: 2021-09-09 12:16:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-09 12:25:16
 * @FilePath: /algorithm/68_full_justify/full_justify.rs
 * @Description: file content
 */

struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let ll = words.len();
        let max_width = max_width as usize;
        let mut queue = Vec::new();
        let mut i = 0;
        let mut count;
        let mut tmp;

        while i < ll {
            count = 0;
            tmp = Vec::new();

            while i < ll && count < max_width {
                // 按max_with将单词分组
                if count + words[i].len() > max_width {
                    break;
                }

                count += words[i].len() + 1;
                tmp.push(words[i].clone());
                i += 1;
            }

            if i == ll {
                // 最后一行，在单词后增加空格
                let last_line = tmp[..].join(&" ");
                let len = last_line.len();
                queue.push(last_line + &" ".repeat(max_width - len));
                break;
            }

            if tmp.len() == 1 {
                // 分组长度为1，在单词后增加空格
                queue.push(tmp[..].join(&"") + &" ".repeat(max_width - count + 1));
            } else {
                let tt = tmp.len() - 1;
                let paddings = max_width - (count - tt - 1);
                let space = paddings / tt; // 每个单词后应该增加的平均空格数
                let mut extra = (paddings % tt) as i32; // 如果每个单词后增加的空格数不能被平均，将额外空格分摊到前几个单词后面
                let res = tmp
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let times = space + if extra <= 0 { 0 } else { 1 };
                        extra -= 1;

                        if i == tt {
                            x
                        } else {
                            x + &" ".repeat(times)
                        }
                    })
                    .collect::<String>();
                queue.push(res);
            }
        }

        queue
    }
}

fn main() {
    {
        let words = vec![
            "This".to_string(),
            "is".to_string(),
            "an".to_string(),
            "example".to_string(),
            "of".to_string(),
            "text".to_string(),
            "justification.".to_string(),
        ];
        let max_width = 16;
        let ans = vec![
            "This    is    an".to_string(),
            "example  of text".to_string(),
            "justification.  ".to_string(),
        ];
        assert_eq!(Solution::full_justify(words, max_width), ans);
    }
    {
        let words = vec![
            "What".to_string(),
            "must".to_string(),
            "be".to_string(),
            "acknowledgment".to_string(),
            "shall".to_string(),
            "be".to_string(),
        ];
        let max_width = 16;
        let ans = vec![
            "What   must   be".to_string(),
            "acknowledgment  ".to_string(),
            "shall be        ".to_string(),
        ];
        assert_eq!(Solution::full_justify(words, max_width), ans);
    }
    {
        let words = vec![
            "Science".to_string(),
            "is".to_string(),
            "what".to_string(),
            "we".to_string(),
            "understand".to_string(),
            "well".to_string(),
            "enough".to_string(),
            "to".to_string(),
            "explain".to_string(),
            "to".to_string(),
            "a".to_string(),
            "computer.".to_string(),
            "Art".to_string(),
            "is".to_string(),
            "everything".to_string(),
            "else".to_string(),
            "we".to_string(),
            "do".to_string(),
        ];
        let max_width = 20;
        let ans = vec![
            "Science  is  what we".to_string(),
            "understand      well".to_string(),
            "enough to explain to".to_string(),
            "a  computer.  Art is".to_string(),
            "everything  else  we".to_string(),
            "do                  ".to_string(),
        ];
        assert_eq!(Solution::full_justify(words, max_width), ans);
    } 
}
