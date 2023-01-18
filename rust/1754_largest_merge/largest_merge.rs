/*
 * @Date: 2022-12-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-24
 * @FilePath: /algorithm/1754_largest_merge/largest_merge.rs
 */

pub fn largest_merge(word1: String, word2: String) -> String {
    let (mut i, mut j, mut ret, word1, word2) = (
        0,
        0,
        String::with_capacity(word1.len() + word2.len()),
        word1.into_bytes(),
        word2.into_bytes(),
    );
    let (m, n) = (word1.len(), word2.len());
    while i < m || j < n {
        if word1[i..] > word2[j..] {
            ret.push(word1[i] as char);
            i += 1;
        } else {
            ret.push(word2[j] as char);
            j += 1;
        }
    }
    ret
}

fn main() {
    {
        let (word1, word2) = (String::from("cabaa"), String::from("bcaaa"));
        let ans = String::from("cbcabaaaaa");
        assert_eq!(largest_merge(word1, word2), ans);
    }

    {
        let (word1, word2) = (String::from("abcabc"), String::from("abdcaba"));
        let ans = String::from("abdcabcabcaba");
        assert_eq!(largest_merge(word1, word2), ans);
    }
}

// word1[i:]: abcabc word2[j:]: abdcaba
// word1[i:]: abcabc word2[j:]: bdcaba
// word1[i:]: abcabc word2[j:]: dcaba
// word1[i:]: abcabc word2[j:]: caba
// word1[i:]: abcabc word2[j:]: aba
// word1[i:]: bcabc word2[j:]: aba
// word1[i:]: cabc word2[j:]: aba
// word1[i:]: abc word2[j:]: aba
// word1[i:]: bc word2[j:]: aba
// word1[i:]: c word2[j:]: aba
// word1[i:]:  word2[j:]: aba
// word1[i:]:  word2[j:]: ba
// word1[i:]:  word2[j:]: a
