/*
 * @Date: 2021-10-16 09:23:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-16 10:10:36
 */

struct Solution;

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let v = num.bytes().map(|u| (u - b'0') as i64).collect::<Vec<_>>();
        let mut answer = Vec::new();
        Self::backtrack(&mut answer, &mut Vec::new(), &v, 0, 0, target as i64);
        answer
    }
    fn backtrack(
        answer: &mut Vec<String>,
        ops: &mut Vec<String>,
        v: &[i64],
        val: i64,
        last: i64,
        target: i64,
    ) {
        if v.is_empty() {
            if val == target {
                answer.push(ops[1..].join(""));
            }
            return;
        }
        let mut n = 0;
        for (i, &d) in v.iter().enumerate() {
            n = n * 10 + d;
            ops.push(String::from("+"));
            ops.push(n.to_string());
            Self::backtrack(answer, ops, &v[i + 1..], val + n, n, target);
            ops.pop();
            ops.pop();
            if !ops.is_empty() {
                ops.push(String::from("-"));
                ops.push(n.to_string());
                Self::backtrack(answer, ops, &v[i + 1..], val - n, -n, target);
                ops.pop();
                ops.pop();
                ops.push(String::from("*"));
                ops.push(n.to_string());
                Self::backtrack(
                    answer,
                    ops,
                    &v[i + 1..],
                    val - last + last * n,
                    last * n,
                    target,
                );
                ops.pop();
                ops.pop();
            }
            if v[0] == 0 {
                break;
            }
        }
    }
}

fn main() {
    assert_eq!(
        Solution::add_operators("123".to_string(), 6),
        vec!["1+2+3", "1*2*3"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::add_operators("232".to_string(), 8),
        vec!["2+3*2", "2*3+2"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::add_operators("105".to_string(), 5),
        vec!["1*0+5", "10-5"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::add_operators("00".to_string(), 0),
        vec!["0+0", "0-0", "0*0"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::add_operators("3456237490".to_string(), 9191),
        Vec::<String>::new()
    );
}
