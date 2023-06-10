/*
 * @Date: 2023-06-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-10
 * @FilePath: /algorithm/rust/1170_num_smaller_by_frequency/num_smaller_by_frequency.rs
 */

pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    fn f(s: String) -> i32 {
        let mut tab = [0; 26];
        s.bytes().for_each(|c| tab[(c - b'a') as usize] += 1);
        for &x in tab.iter() {
            if x > 0 {
                return x;
            }
        }
        return 0;
    }
    let mut words: Vec<_> = words.into_iter().map(|s| (f(s), 0)).collect();
    words.sort_unstable();
    let mut i = 0;
    let mut j = 0;
    while j < words.len() {
        while j < words.len() && words[i].0 == words[j].0 {
            words[i].1 += 1;
            j += 1;
        }
        if j < words.len() {
            i += 1;
            words[i].0 = words[j].0;
        }
    }
    words.truncate(i + 1);
    for i in 1..words.len() {
        words[i].1 += words[i - 1].1;
    }
    let total = words.last().unwrap().1;
    queries
        .into_iter()
        .map(|s| {
            total
                - match words.binary_search_by_key(&f(s), |w| w.0) {
                    Ok(i) => words[i].1,
                    Err(i) => {
                        if i == 0 {
                            0
                        } else {
                            words[i - 1].1
                        }
                    }
                }
        })
        .collect()
}

fn main() {
    {
        let queries = ["cbd"].iter().map(|s| s.to_string()).collect();
        let words = ["zaaaz"].iter().map(|s| s.to_string()).collect();
        let ans = vec![1];
        assert_eq!(num_smaller_by_frequency(queries, words), ans);
    }

    {
        let queries = ["bbb", "cc"].iter().map(|s| s.to_string()).collect();
        let words = ["a", "aa", "aaa", "aaaa"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = vec![1, 2];
        assert_eq!(num_smaller_by_frequency(queries, words), ans);
    }
}
