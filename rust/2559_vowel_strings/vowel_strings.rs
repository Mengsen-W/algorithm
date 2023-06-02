/*
 * @Date: 2023-06-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-02
 * @FilePath: /algorithm/rust/2559_vowel_strings/vowel_strings.rs
 */

pub fn vowel_strings(words: Vec<String>, quest: Vec<Vec<i32>>) -> Vec<i32> {
    let qs: usize = quest.len();
    let ws: usize = words.len();
    let mut res: Vec<i32> = vec![0; qs];
    let mut pre_sum: Vec<i32> = vec![0; ws];
    for (index, string) in words.iter().enumerate() {
        let chr = string.get(0..1).unwrap();
        let chr_back = string.get((string.len() - 1)..).unwrap();

        pre_sum[index] = if (chr == "a" || chr == "e" || chr == "i" || chr == "o" || chr == "u")
            && (chr_back == "a"
                || chr_back == "e"
                || chr_back == "i"
                || chr_back == "o"
                || chr_back == "u")
        {
            1
        } else {
            0
        } + if index == 0 { 0 } else { pre_sum[index - 1] };
    }
    for (index, seg) in quest.iter().enumerate() {
        res[index] = pre_sum[seg[1] as usize]
            - if seg[0] == 0 {
                0
            } else {
                pre_sum[seg[0] as usize - 1]
            };
    }
    res
}

fn main() {
    {
        let words = vec!["aba", "bcb", "ece", "aa", "e"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let queries = [[0, 2], [1, 4], [1, 1]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = vec![2, 3, 0];
        assert_eq!(vowel_strings(words, queries), ans);
    }

    {
        let words = vec!["a", "e", "i"].iter().map(|s| s.to_string()).collect();
        let queries = [[0, 2], [0, 1], [2, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = vec![3, 2, 1];
        assert_eq!(vowel_strings(words, queries), ans);
    }
}
