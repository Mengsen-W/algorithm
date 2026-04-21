struct Solution;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = dominoes.chars().collect::<Vec<char>>();
        let (n, mut i) = (dominoes.len(), 0);
        let mut left = 'L';
        while i < n {
            let mut j = i;
            while j < n && dominoes[j] == '.' {
                j += 1;
            }
            let right = if j < n { dominoes[j] } else { 'R' };
            if left == right {
                while i < j {
                    dominoes[i] = left;
                    i += 1;
                }
            } else if left == 'R' && right == 'L' {
                let mut k = j - 1;
                while i < k {
                    dominoes[i] = 'R';
                    i += 1;
                    dominoes[k] = 'L';
                    k -= 1;
                }
            }
            left = right;
            i = j + 1;
        }
        dominoes.iter().collect::<String>()
    }
}

fn main() {
    let tests = vec![("RR.L", "RR.L"), (".L.R...LR..L..", "LL.RR.LLRRLL..")];

    for (dominoes, ans) in tests {
        assert_eq!(
            Solution::push_dominoes(dominoes.to_string()),
            ans.to_string()
        );
    }
}
