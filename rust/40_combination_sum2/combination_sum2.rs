struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut freq: Vec<(i32, i32)> = Vec::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut sequence: Vec<i32> = Vec::new();

        let mut candidates = candidates;
        candidates.sort();
        for &num in &candidates {
            if freq.is_empty() || num != freq.last().unwrap().0 {
                freq.push((num, 1));
            } else {
                freq.last_mut().unwrap().1 += 1;
            }
        }

        fn dfs(
            pos: usize,
            rest: i32,
            freq: &Vec<(i32, i32)>,
            sequence: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if rest == 0 {
                ans.push(sequence.clone());
                return;
            }
            if pos == freq.len() || rest < freq[pos].0 {
                return;
            }

            dfs(pos + 1, rest, freq, sequence, ans);

            let most = (rest / freq[pos].0).min(freq[pos].1);
            for i in 1..=most {
                sequence.push(freq[pos].0);
                dfs(pos + 1, rest - i * freq[pos].0, freq, sequence, ans);
            }
            for _ in 1..=most {
                sequence.pop();
            }
        }

        dfs(0, target, &freq, &mut sequence, &mut ans);
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![10, 1, 2, 7, 6, 1, 5],
            8,
            vec![vec![2, 6], vec![1, 7], vec![1, 2, 5], vec![1, 1, 6]],
        ),
        (vec![2, 5, 2, 1, 2], 5, vec![vec![5], vec![1, 2, 2]]),
    ];

    for (candidates, target, ans) in tests {
        assert_eq!(Solution::combination_sum2(candidates, target), ans);
    }
}
