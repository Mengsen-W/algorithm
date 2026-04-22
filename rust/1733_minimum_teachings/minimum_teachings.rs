struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut cncon = HashSet::new();
        for friendship in friendships {
            let mut mp = HashSet::new();
            let mut conm = false;
            for &lan in &languages[friendship[0] as usize - 1] {
                mp.insert(lan);
            }
            for &lan in &languages[friendship[1] as usize - 1] {
                if mp.contains(&lan) {
                    conm = true;
                    break;
                }
            }

            if !conm {
                cncon.insert(friendship[0] - 1);
                cncon.insert(friendship[1] - 1);
            }
        }

        let mut max_cnt = 0;
        let mut cnt = HashMap::new();
        for &person in &cncon {
            for &lan in &languages[person as usize] {
                *cnt.entry(lan).or_insert(0) += 1;
                max_cnt = max_cnt.max(*cnt.get(&lan).unwrap());
            }
        }

        cncon.len() as i32 - max_cnt
    }
}

fn main() {
    let tests = vec![
        (
            2,
            vec![vec![1], vec![2], vec![1, 2]],
            vec![vec![1, 2], vec![1, 3], vec![2, 3]],
            1,
        ),
        (
            3,
            vec![vec![2], vec![1, 3], vec![1, 2], vec![3]],
            vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]],
            2,
        ),
    ];

    for (n, languages, friendships, expected) in tests {
        assert_eq!(
            Solution::minimum_teachings(n, languages, friendships),
            expected
        );
    }
}
