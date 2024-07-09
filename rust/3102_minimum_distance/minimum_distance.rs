struct Solution;

impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut sx: Vec<(i32, usize)> = Vec::with_capacity(n);
        let mut sy: Vec<(i32, usize)> = Vec::with_capacity(n);
        for (i, point) in points.iter().enumerate() {
            let x = point[0];
            let y = point[1];
            sx.push((x - y, i));
            sy.push((x + y, i));
        }
        sx.sort_unstable_by_key(|pair| pair.0);
        sy.sort_unstable_by_key(|pair| pair.0);
        let max_val1 = sx[n - 1].0 - sx[0].0;
        let max_val2 = sy[n - 1].0 - sy[0].0;
        let mut res = std::i32::MAX;
        if max_val1 >= max_val2 {
            let i = sx[0].1;
            let j = sx[n - 1].1;
            // 去掉 i 后的最大曼哈顿距离
            res = res.min(std::cmp::max(Self::remove(&sx, i), Self::remove(&sy, i)));
            // 去掉 j 后的最大曼哈顿距离
            res = res.min(std::cmp::max(Self::remove(&sx, j), Self::remove(&sy, j)));
        } else {
            let i = sy[0].1;
            let j = sy[n - 1].1;
            // 去掉 i 后的最大曼哈顿距离
            res = res.min(std::cmp::max(Self::remove(&sx, i), Self::remove(&sy, i)));
            // 去掉 j 后的最大曼哈顿距离
            res = res.min(std::cmp::max(Self::remove(&sx, j), Self::remove(&sy, j)));
        }
        res
    }

    fn remove(arr: &Vec<(i32, usize)>, i: usize) -> i32 {
        let n = arr.len();
        if arr[0].1 == i {
            return arr[n - 1].0 - arr[1].0;
        } else if arr[n - 1].1 == i {
            return arr[n - 2].0 - arr[0].0;
        } else {
            return arr[n - 1].0 - arr[0].0;
        }
    }
}

fn main() {
    let tests = vec![
        (vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 4]], 12),
        (vec![vec![1, 1], vec![1, 1], vec![1, 1]], 0),
    ];

    for (points, ans) in tests {
        assert_eq!(Solution::minimum_distance(points), ans);
    }
}
