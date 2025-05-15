struct Solution;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut res = Vec::new();
        let mut freq = HashMap::new();
        // 统计数字数组中各数字的出现次数
        for &d in &digits {
            *freq.entry(d).or_insert(0) += 1;
        }
        // 枚举所有三位偶数
        for i in (100..1000).step_by(2) {
            let mut freq1 = HashMap::new();
            let mut num = i;
            // 统计当前偶数的每一位数字的出现次数
            while num > 0 {
                let d = num % 10;
                *freq1.entry(d).or_insert(0) += 1;
                num /= 10;
            }
            // 检查是否满足条件
            let mut is_valid = true;
            for (&d, &count) in &freq1 {
                if freq.get(&d).unwrap_or(&0) < &count {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                res.push(i);
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            vec![2, 1, 3, 0],
            vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320],
        ),
        (vec![2, 2, 8, 8, 2], vec![222, 228, 282, 288, 822, 828, 882]),
        (vec![3, 7, 5], vec![]),
    ];

    for (digits, ans) in tests {
        assert_eq!(Solution::find_even_numbers(digits), ans);
    }
}
