/*
 * @Date: 2021-11-16 01:27:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-16 01:32:17
 * @FilePath: /algorithm/391_is_rectangle_cover/is_rectangle_cover.rs
 * @Description: file content
 */

struct Solution;

impl Solution {
    fn help(mp: &mut std::collections::HashMap<(i32, i32), i32>, x: i32, y: i32) {
        if let Some(cnt) = mp.get_mut(&(x, y)) {
            *cnt += 1;
        } else {
            mp.insert((x, y), 1);
        }
    }

    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, 0, 0);
        let mut area = 0;

        let mut mp = std::collections::HashMap::new();

        for rect in rectangles.iter() {
            let (x, y, a, b) = (rect[0], rect[1], rect[2], rect[3]);
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(a);
            max_y = max_y.max(b);
            area += (a - x) * (b - y);

            Solution::help(&mut mp, x, y);
            Solution::help(&mut mp, x, b);
            Solution::help(&mut mp, a, y);
            Solution::help(&mut mp, a, b);
        }

        if (max_x - min_x) * (max_y - min_y) != area {
            return false;
        }

        for (k, v) in mp {
            if k == (min_x, min_y)
                || k == (max_x, min_y)
                || k == (min_x, max_y)
                || k == (max_x, max_y)
            {
                if v != 1 {
                    return false;
                }
            } else {
                if v != 2 && v != 4 {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4],
            vec![1, 3, 2, 4],
            vec![2, 3, 3, 4]
        ]),
        true
    );
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 2, 3],
            vec![1, 3, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4]
        ]),
        false
    );
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![3, 2, 4, 4]
        ]),
        false
    );
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![2, 2, 4, 4]
        ]),
        false
    );
}
