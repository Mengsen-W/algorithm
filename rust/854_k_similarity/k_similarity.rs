/*
 * @Date: 2022-09-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-21
 * @FilePath: /algorithm/854_k_similarity/k_similarity.rs
 */

pub fn k_similarity(s1: String, s2: String) -> i32 {
    use std::collections::{HashSet, VecDeque};
    let n = s1.len();
    let a = s1.chars().collect::<Vec<_>>();
    let b = s2.chars().collect::<Vec<_>>();
    let mut ans = 0;
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(a);

    while queue.len() > 0 {
        let queue_size = queue.len();

        for _ in 0..queue_size {
            if let Some(mut v) = queue.pop_front() {
                if v == b {
                    return ans;
                }

                let mut i = 0;

                while v[i] == b[i] {
                    i += 1;
                }

                let mut j = i + 1;

                while j < n {
                    if v[j] == b[i] {
                        v.swap(i, j);

                        if seen.insert(v.clone()) {
                            queue.push_back(v.clone());
                        }

                        v.swap(i, j);
                    }

                    j += 1;
                }
            }
        }

        ans += 1;
    }

    ans
}

fn main() {
    {
        let s1 = String::from("ab");
        let s2 = String::from("ba");
        let ans = 1;
        assert_eq!(k_similarity(s1, s2), ans);
    }

    {
        let s1 = String::from("abc");
        let s2 = String::from("bca");
        let ans = 2;
        assert_eq!(k_similarity(s1, s2), ans);
    }
}
