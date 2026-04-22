struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut ans = n; // 先给每人分一个
        let mut i = 0;
        while i < n {
            let start = if i > 0 && ratings[i - 1] < ratings[i] {
                i - 1
            } else {
                i
            };

            // 找严格递增段
            while i + 1 < n && ratings[i] < ratings[i + 1] {
                i += 1;
            }
            let top = i; // 峰顶

            // 找严格递减段
            while i + 1 < n && ratings[i] > ratings[i + 1] {
                i += 1;
            }

            let inc = top - start; // start 到 top 严格递增
            let dec = i - top; // top 到 i 严格递减
            ans += (inc * (inc - 1) + dec * (dec - 1)) / 2 + inc.max(dec);
            i += 1;
        }
        ans as _
    }
}

fn main() {
    let tests = vec![(vec![1, 0, 2], 5), (vec![1, 2, 2], 4)];

    for (ratings, ans) in tests {
        assert_eq!(Solution::candy(ratings), ans);
    }
}
