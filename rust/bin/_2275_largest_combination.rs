struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        // 计算从低到高第 k 个二进制位数值为 1 的元素个数
        fn maxlen(candidates: &Vec<i32>, k: i32) -> i32 {
            let mut res = 0;
            for &num in candidates.iter() {
                if num & (1 << k) != 0 {
                    res += 1;
                }
            }
            res
        }

        let mut res = 0;
        for i in 0..24 {
            // 遍历二进制位
            res = res.max(maxlen(&candidates, i));
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![16, 17, 71, 62, 12, 24, 14], 4), (vec![8, 8], 2)];

    for (candidates, ans) in tests {
        assert_eq!(Solution::largest_combination(candidates), ans);
    }
}
