/*
 * @Date: 2023-03-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-30
 * @FilePath: /algorithm/rust/1637_max_width_of_vertical_area/max_width_of_vertical_area.rs
 */

pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by(|a, b| a[0].cmp(&b[0]));
    (1..points.len()).fold(0, |ret, i| ret.max(points[i][0] - points[i - 1][0]))
}

fn main() {
    {
        let points = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];
        let ans = 1;
        assert_eq!(max_width_of_vertical_area(points), ans);
    }

    {
        let points = vec![
            vec![3, 1],
            vec![9, 0],
            vec![1, 0],
            vec![1, 4],
            vec![5, 3],
            vec![8, 8],
        ];
        let ans = 3;
        assert_eq!(max_width_of_vertical_area(points), ans);
    }
}
