/*
 * @Date: 2022-07-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-29
 * @FilePath: /algorithm/593_valid_square/valid_square.rs
 */

pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    fn check_length(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
        (v1[0] * v1[0] + v1[1] * v1[1]) == (v2[0] * v2[0] + v2[1] * v2[1])
    }

    fn check_mid_point(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>, p4: &Vec<i32>) -> bool {
        (p1[0] + p2[0]) == (p3[0] + p4[0]) && (p1[1] + p2[1]) == (p3[1] + p4[1])
    }

    fn cal_cos(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
        (v1[0] * v2[0] + v1[1] * v2[1]) == 0
    }

    fn help(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>, p4: &Vec<i32>) -> bool {
        let v1 = vec![p1[0] - p2[0], p1[1] - p2[1]];
        let v2 = vec![p3[0] - p4[0], p3[1] - p4[1]];

        check_mid_point(p1, p2, p3, p4) && check_length(&v1, &v2) && cal_cos(&v1, &v2)
    }

    if p1 == p2 || p1 == p3 || p1 == p4 {
        return false;
    }
    if help(&p1, &p2, &p3, &p4) || help(&p1, &p3, &p2, &p4) || help(&p1, &p4, &p2, &p3) {
        return true;
    }
    false
}

fn main() {
    {
        let p1 = vec![0, 0];
        let p2 = vec![1, 1];
        let p3 = vec![1, 0];
        let p4 = vec![0, 1];
        assert_eq!(valid_square(p1, p2, p3, p4), true);
    }
    {
        let p1 = vec![0, 0];
        let p2 = vec![1, 1];
        let p3 = vec![1, 0];
        let p4 = vec![0, 12];
        assert_eq!(valid_square(p1, p2, p3, p4), false);
    }
    {
        let p1 = vec![1, 0];
        let p2 = vec![-1, 0];
        let p3 = vec![0, 1];
        let p4 = vec![0, -1];
        assert_eq!(valid_square(p1, p2, p3, p4), true);
    }
}
