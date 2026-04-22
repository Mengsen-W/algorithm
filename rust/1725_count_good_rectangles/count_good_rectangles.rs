/*
 * @Date: 2022-02-04 00:55:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-04 01:08:31
 */

pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let (mut res, mut max_len) = (0, 0);
    for rectangle in &rectangles {
        let (l, w) = (rectangle[0], rectangle[1]);
        let k = l.min(w);
        if k == max_len {
            res += 1;
        } else if k > max_len {
            res = 1;
            max_len = k;
        }
    }

    res
}

fn main() {
    assert_eq!(
        count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]]),
        3
    );
    assert_eq!(
        count_good_rectangles(vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]]),
        3
    );
}
