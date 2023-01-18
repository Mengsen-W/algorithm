/*
 * @Date: 2022-05-14 09:03:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-14 10:14:38
 * @FilePath: /algorithm/691_min_stickers/min_stickers.rs
 */

pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
    let n = target.len();
    let mut t_mask = vec![0; 26];

    for (i, c) in target.bytes().enumerate() {
        t_mask[(c - b'a') as usize] |= 1 << i;
    }

    let mut masked_words = vec![];
    for s in stickers {
        let mut masks = vec![];
        for c in s.bytes() {
            let i = (c - b'a') as usize;
            if t_mask[i] > 0 {
                masks.push(t_mask[i]);
            }
        }
        masked_words.push(masks);
    }

    let mut dp = vec![usize::MAX; 1 << n];
    dp[0] = 0;

    for i in 0..dp.len() {
        if dp[i] == usize::MAX {
            continue;
        }
        for w in &masked_words {
            let mut nxt = i;

            for m in w {
                let mut b = *m;
                while b > 0 {
                    let pos = b - ((b - 1) & b);
                    if nxt & pos == 0 {
                        nxt |= pos;
                        break;
                    }
                    b -= pos;
                }
            }
            dp[nxt] = dp[nxt].min(dp[i] + 1);
        }
    }

    *dp.last().unwrap() as i32
}

fn main() {
    assert_eq!(
        min_stickers(
            vec!["with", "example", "science"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            "thehat".to_string()
        ),
        3
    );

    assert_eq!(
        min_stickers(
            vec!["notice", "possible"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            "basicbasic".to_string()
        ),
        -1
    );
}
