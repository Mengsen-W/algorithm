struct Solution;

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut total_area: i64 = 0;
        let mut events: Vec<(i32, i32, i32)> = Vec::new();

        for sq in &squares {
            let y = sq[1];
            let l = sq[2];
            total_area += l as i64 * l as i64;
            events.push((y, l, 1));
            events.push((y + l, l, -1));
        }

        // 按y坐标排序
        events.sort_by_key(|&(y, _, _)| y);

        let mut covered_width: f64 = 0.0; // 当前扫描线下所有底边之和
        let mut curr_area: f64 = 0.0; // 当前累计面积
        let mut prev_height: f64 = 0.0; // 前一个扫描线的高度

        for (y, l, delta) in events {
            let diff = y as f64 - prev_height;
            // 两条扫描线之间新增的面积
            let area = covered_width * diff;
            // 如果加上这部分面积超过总面积的一半
            if 2.0 * (curr_area + area) >= total_area as f64 {
                return prev_height + (total_area as f64 - 2.0 * curr_area) / (2.0 * covered_width);
            }
            // 更新宽度：开始事件加宽度，结束事件减宽度
            covered_width += (delta * l) as f64;
            curr_area += area;
            prev_height = y as f64;
        }

        0.0
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 0, 1], vec![2, 2, 1]], 1.0),
        (vec![vec![0, 0, 2], vec![1, 1, 1]], 1.1666666666666667),
    ];

    for (test, expected) in tests {
        assert_eq!(Solution::separate_squares(test), expected);
    }
}
