/*
 * @Date: 2022-05-07 06:31:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-07 06:43:21
 * @FilePath: /algorithm/433_min_mutation/min_mutation.rs
 */

pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
    use std::collections::HashSet;
    if !bank.contains(&end) {
        return -1;
    }

    let mut st = HashSet::<String>::new();
    let mut en = HashSet::<String>::new();
    let mst = Box::new(HashSet::<String>::new());
    let men = Box::new(HashSet::<String>::new());
    let mut bank_list = HashSet::<String>::new();
    bank.into_iter().for_each(|item| {
        bank_list.insert(item);
    });
    st.insert(start);
    en.insert(end);

    fn bfs(
        end: &HashSet<String>,
        start: &HashSet<String>,
        men: Box<HashSet<String>>,
        mut mst: Box<HashSet<String>>,
        bank: &HashSet<String>,
        level: i32,
    ) -> i32 {
        if start.is_empty() {
            return -1;
        }

        if start.len() > end.len() {
            return bfs(&start, &end, mst, men, &bank, level);
        }

        let mut temp: HashSet<String> = HashSet::<String>::new();
        let index = ['A', 'C', 'G', 'T'];
        for i in start {
            for j in 0..8 {
                for g in index {
                    let str_temp: String = i
                        .chars()
                        .enumerate()
                        .map(|(f, z)| if f == j { g.to_string() } else { z.to_string() })
                        .collect();

                    if end.contains(&str_temp) {
                        return level + 1;
                    }
                    if mst.contains(&str_temp) {
                        continue;
                    }
                    if bank.contains(&str_temp) {
                        temp.insert(str_temp.to_string().clone());
                        mst.insert(str_temp.clone());
                    }
                }
            }
        }

        return bfs(&end, &temp, men, mst, &bank, level + 1);
    }

    bfs(&en, &st, men, mst, &bank_list, 0)
}

fn main() {
    assert_eq!(
        min_mutation(
            String::from("AACCGGTT"),
            String::from("AAACGGTA"),
            vec![
                String::from("AACCGGTA"),
                String::from("AACCGCTA"),
                String::from("AAACGGTA")
            ]
        ),
        2
    );
    assert_eq!(
        min_mutation(
            String::from("AACCGGTT"),
            String::from("AACCGGTA"),
            vec![String::from("AACCGGTA")]
        ),
        1
    );
    assert_eq!(
        min_mutation(
            String::from("AAAAACCC"),
            String::from("AACCCCCC"),
            vec![
                String::from("AAAACCCC"),
                String::from("AAACCCCC"),
                String::from("AACCCCCC")
            ]
        ),
        3
    );
}
