struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut ret = 0;
        let strs_arr = strs
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        for i in 0..strs[0].len() {
            for j in 1..strs.len() {
                if strs_arr[j - 1][i] > strs_arr[j][i] {
                    ret += 1;
                    break;
                }
            }
        }
        ret
    }
}

fn main() {
    let tests = vec![
        (vec!["cba", "daf", "ghi"], 1),
        (vec!["a", "b"], 0),
        (vec!["zyx", "wvu", "tsr"], 3),
    ];

    for (strs, expected) in tests {
        assert_eq!(
            Solution::min_deletion_size(strs.iter().map(|s| s.to_string()).collect()),
            expected
        );
    }
}
