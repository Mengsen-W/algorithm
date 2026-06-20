struct Solution;

impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut r: Vec<Vec<i32>> = restrictions.clone();
        // 增加限制 (1, 0)
        r.push(vec![1, 0]);

        // 按位置排序
        r.sort_by(|a, b| a[0].cmp(&b[0]));

        // 增加限制 (n, n-1)
        if r.last().unwrap()[0] != n {
            r.push(vec![n, n - 1]);
        }

        let m = r.len();

        // 从左向右传递限制
        for i in 1..m {
            let dist = r[i][0] - r[i - 1][0];
            r[i][1] = r[i][1].min(r[i - 1][1] + dist);
        }

        // 从右向左传递限制
        for i in (0..m - 1).rev() {
            let dist = r[i + 1][0] - r[i][0];
            r[i][1] = r[i][1].min(r[i + 1][1] + dist);
        }

        let mut ans = 0;
        for i in 0..m - 1 {
            let dist = r[i + 1][0] - r[i][0];
            let best = (dist + r[i][1] + r[i + 1][1]) / 2;
            ans = ans.max(best);
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (5, vec![vec![2, 1], vec![4, 1]], 2),
        (6, vec![], 5),
        (10, vec![vec![5, 3], vec![2, 5], vec![7, 4], vec![10, 3]], 5),
    ];

    for (n, restrictions, expected) in tests {
        assert_eq!(Solution::max_building(n, restrictions), expected);
    }
}
