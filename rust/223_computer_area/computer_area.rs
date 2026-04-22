/*
 * @Date: 2021-09-30 09:25:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-30 09:40:14
 */

struct Solution;

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let (area1, area2) = ((ax2 - ax1) * (ay2 - ay1), (bx2 - bx1) * (by2 - by1));
        let (overlap_width, overlap_height) =
            (ax2.min(bx2) - ax1.max(bx1), ay2.min(by2) - ay1.max(by1));
        let overlap_area = overlap_width.max(0) * overlap_height.max(0);
        return area1 + area2 - overlap_area;
    }
}

fn main() {
    {
        let ax1 = -3;
        let ay1 = 0;
        let ax2 = 3;
        let ay2 = 4;
        let bx1 = 0;
        let by1 = -1;
        let bx2 = 9;
        let by2 = 2;
        let ans = 45;
        assert_eq!(
            Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2),
            ans
        );
    }
    {
        let ax1 = -2;
        let ay1 = -2;
        let ax2 = 2;
        let ay2 = 2;
        let bx1 = -2;
        let by1 = -2;
        let bx2 = 2;
        let by2 = 2;
        let ans = 16;
        assert_eq!(
            Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2),
            ans
        );
    }
}
