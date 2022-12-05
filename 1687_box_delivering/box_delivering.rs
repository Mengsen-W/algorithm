/*
 * @Date: 2022-12-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-05
 * @FilePath: /algorithm/1687_box_delivering/box_delivering.rs
 */

pub fn box_delivering(boxes: Vec<Vec<i32>>, _: i32, max_boxes: i32, max_weight: i32) -> i32 {
    let (max_boxes, max_weight) = (max_boxes as usize, max_weight as i64);
    let n = boxes.len();
    let (mut p, mut w, mut neg) = (vec![0; n + 1], vec![0; n + 1], vec![0; n + 1]);
    let mut ww = vec![0 as i64; n + 1];

    for i in 1..=n {
        p[i] = boxes[i - 1][0];
        w[i] = boxes[i - 1][1];
        if i > 1 {
            neg[i] = neg[i - 1] + if p[i - 1] != p[i] { 1 } else { 0 };
        }
        ww[i] = ww[i - 1] + w[i] as i64;
    }
    use std::collections::VecDeque;
    let mut opt = VecDeque::from([0]);
    let (mut f, mut g) = (vec![0; n + 1], vec![0; n + 1]);
    for i in 1..=n {
        while i - *opt.front().unwrap() > max_boxes
            || (ww[i] - ww[*opt.front().unwrap()]) > max_weight
        {
            opt.pop_front();
        }

        f[i] = g[*opt.front().unwrap()] + neg[i] + 2;

        if i != n {
            g[i] = f[i] - neg[i + 1];
            while !opt.is_empty() && g[i] <= g[*opt.back().unwrap()] {
                opt.pop_back();
            }
            opt.push_back(i);
        }
    }
    f[n]
}

fn main() {
    {
        let boxes = vec![vec![1, 1], vec![2, 1], vec![1, 1]];
        let ports_count = 2;
        let max_boxes = 3;
        let max_weight = 3;
        let ans = 4;
        assert_eq!(
            box_delivering(boxes, ports_count, max_boxes, max_weight),
            ans
        );
    }

    {
        let boxes = vec![vec![1, 2], vec![3, 3], vec![3, 1], vec![3, 1], vec![2, 4]];
        let ports_count = 3;
        let max_boxes = 3;
        let max_weight = 6;
        let ans = 6;
        assert_eq!(
            box_delivering(boxes, ports_count, max_boxes, max_weight),
            ans
        );
    }

    {
        let boxes = vec![
            vec![1, 4],
            vec![1, 2],
            vec![2, 1],
            vec![2, 1],
            vec![3, 2],
            vec![3, 4],
        ];
        let ports_count = 3;
        let max_boxes = 6;
        let max_weight = 7;
        let ans = 6;
        assert_eq!(
            box_delivering(boxes, ports_count, max_boxes, max_weight),
            ans
        );
    }

    {
        let boxes = vec![
            vec![2, 4],
            vec![2, 5],
            vec![3, 1],
            vec![3, 2],
            vec![3, 7],
            vec![3, 1],
            vec![4, 4],
            vec![1, 3],
            vec![5, 2],
        ];
        let ports_count = 5;
        let max_boxes = 5;
        let max_weight = 7;
        let ans = 14;
        assert_eq!(
            box_delivering(boxes, ports_count, max_boxes, max_weight),
            ans
        );
    }
}
