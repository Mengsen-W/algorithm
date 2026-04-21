/*
 * @Date: 2023-04-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-08
 * @FilePath: /algorithm/rust/1125_smallest_sufficient_team/smallest_sufficient_team.rs
 */

pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
    use std::collections::HashMap;
    // 状态压缩,首先是需要的技术栈的编码,使得可以通过打表得到需求技术的对应编码
    let mut map = HashMap::new();
    (0..req_skills.len()).for_each(|i| {
        map.insert(req_skills[i].clone(), i);
    });
    let mut skills = Vec::with_capacity(people.len());
    people.into_iter().for_each(|ones| {
        let mut mask = 0;
        ones.into_iter().for_each(|skill| {
            let idx = *map.get(&skill).unwrap();
            mask |= 1 << idx;
        });
        skills.push(mask);
    });
    // 定义dp为技术栈为i情况下的最小用户集合,使用下标存储
    let all = 1 << req_skills.len();
    let mut dp = vec![Vec::new(); all];
    (0..all).for_each(|i| {
        if i > 0 && dp[i].len() == 0 {
            () // 还是不能单独作为continue使用
        } else {
            for j in 0..skills.len() {
                if skills[j] == 0 {
                    continue;
                }
                let next = i | skills[j];
                if dp[next].len() == 0 || dp[next].len() > dp[i].len() + 1 {
                    dp[next] = dp[i].clone();
                    dp[next].push(j);
                }
            }
        }
    });
    dp[all - 1].iter().map(|i| *i as i32).collect::<Vec<i32>>()
}

fn main() {
    {
        let req_skills = ["java", "nodejs", "reactjs"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let people = [vec!["java"], vec!["nodejs"], vec!["nodejs", "reactjs"]]
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect();
        let ans = vec![0, 2];
        assert_eq!(smallest_sufficient_team(req_skills, people), ans);
    }

    {
        let req_skills = ["algorithms", "math", "java", "reactjs", "csharp", "aws"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let people = [
            vec!["algorithms", "math", "java"],
            vec!["algorithms", "math", "reactjs"],
            vec!["java", "csharp", "aws"],
            vec!["reactjs", "csharp"],
            vec!["csharp", "math"],
            vec!["aws", "java"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();
        let ans = vec![1, 2];
        assert_eq!(smallest_sufficient_team(req_skills, people), ans);
    }
}
