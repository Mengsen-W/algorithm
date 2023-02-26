/*
 * @Date: 2023-02-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-26
 * @FilePath: /algorithm/rust/1255_max_score_words/max_score_words.rs
 */

pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut cnt = vec![0; 26];
    for ele in letters {
        cnt[(ele as u8 - b'a') as usize] += 1;
    }

    let mut ans = 0;

    'outer: for s in 0..1 << words.len() {
        let mut cur_cnt = cnt.clone();
        let mut cur_score = 0;

        for i in 0..words.len() {
            if s >> i & 1 == 1 {
                for b in words[i].bytes() {
                    let idx = (b - b'a') as usize;
                    if cur_cnt[idx] == 0 {
                        continue 'outer;
                    }
                    cur_cnt[idx] -= 1;
                    cur_score += score[idx];
                }
            }
        }

        ans = ans.max(cur_score);
    }

    ans
}

fn main() {
    {
        let words = vec!["dog", "cat", "dad", "good"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let letters = vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'];
        let score = vec![
            1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let ans = 23;
        assert_eq!(max_score_words(words, letters, score), ans);
    }

    {
        let words = vec!["xxxz", "ax", "bx", "cx"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let letters = vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'];
        let score = vec![
            4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
        ];
        let ans = 27;
        assert_eq!(max_score_words(words, letters, score), ans);
    }

    {
        let words = vec!["leetcode"].iter().map(|s| s.to_string()).collect();
        let letters = vec!['l', 'e', 't', 'c', 'o', 'd'];
        let score = vec![
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        ];
        let ans = 0;
        assert_eq!(max_score_words(words, letters, score), ans);
    }
}
