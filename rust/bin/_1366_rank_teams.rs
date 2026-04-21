struct Solution;

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        use std::collections::HashMap;
        // 初始化哈希映射
        let mut ranking: HashMap<char, Vec<i32>> = HashMap::new();
        for vid in votes[0].chars() {
            ranking.entry(vid).or_insert(vec![0; votes[0].len()]);
        }
        // 遍历统计
        for vote in votes {
            for (i, c) in vote.chars().enumerate() {
                if let Some(rank) = ranking.get_mut(&c) {
                    rank[i] += 1;
                }
            }
        }
        // 取出所有的键值对
        let mut result: Vec<(char, Vec<i32>)> = ranking.into_iter().collect();
        // 排序
        result.sort_by(|a, b| {
            for i in 0..a.1.len() {
                if a.1[i] != b.1[i] {
                    return b.1[i].cmp(&a.1[i]);
                }
            }
            a.0.cmp(&b.0)
        });

        // 构建结果字符串
        let mut ans = String::new();
        for (vid, _) in result {
            ans.push(vid);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec!["ABC", "ACB", "ABC", "ACB", "ACB"], "ACB"),
        (vec!["WXYZ", "XYZW"], "XWYZ"),
        (
            vec!["ZMNAGUEDSJYLBOPHRQICWFXTVK"],
            "ZMNAGUEDSJYLBOPHRQICWFXTVK",
        ),
    ];

    for (votes, ans) in tests {
        assert_eq!(
            Solution::rank_teams(votes.iter().map(|s| s.to_string()).collect()),
            ans
        );
    }
}
