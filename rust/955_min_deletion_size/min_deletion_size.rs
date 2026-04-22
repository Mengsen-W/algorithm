struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        // cuts[i] is True : we don't need to check col[i] <= col[i+1]
        let mut cuts = vec![false; strs.len() - 1];
        let mut ans = 0;

        for j in 0..strs[0].len() {
            let mut can_keep = true;
            for i in 0..strs.len() - 1 {
                if !cuts[i] {
                    let char_i = strs[i].chars().nth(j).unwrap();
                    let char_i1 = strs[i + 1].chars().nth(j).unwrap();
                    if char_i > char_i1 {
                        can_keep = false;
                        break;
                    }
                }
            }
            if can_keep {
                for i in 0..strs.len() - 1 {
                    let char_i = strs[i].chars().nth(j).unwrap();
                    let char_i1 = strs[i + 1].chars().nth(j).unwrap();
                    if char_i < char_i1 {
                        cuts[i] = true;
                    }
                }
            } else {
                ans += 1;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec!["ca", "bb", "ac"], 1),
        (vec!["xc", "yb", "za"], 0),
        (vec!["zyx", "wvu", "tsr"], 3),
    ];

    for (strs, ans) in tests {
        assert_eq!(
            Solution::min_deletion_size(strs.iter().map(|s| s.to_string()).collect()),
            ans
        );
    }
}
