struct Solution;

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        let mut arr = Vec::new();
        Self::dfs(&mut arr, n, &mut res);
        res
    }

    fn dfs(arr: &mut Vec<char>, n: i32, res: &mut Vec<String>) {
        if arr.len() == n as usize {
            res.push(arr.iter().collect());
            return;
        }
        if arr.is_empty() || arr[arr.len() - 1] == '1' {
            arr.push('0');
            Self::dfs(arr, n, res);
            arr.pop();
        }
        arr.push('1');
        Self::dfs(arr, n, res);
        arr.pop();
    }
}

fn main() {
    let tests = vec![
        (3, vec!["010", "011", "101", "110", "111"]),
        (1, vec!["0", "1"]),
    ];

    for (n, ans) in tests {
        assert_eq!(
            Solution::valid_strings(n),
            ans.iter().map(|s| s.to_string()).collect::<Vec<String>>()
        );
    }
}
