/*
 * @Date: 2021-06-24 08:54:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-24 09:30:21
 */

fn max_points(points: Vec<Vec<i32>>) -> i32 {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    let n = points.len() as i32;
    if n <= 2 {
        return n;
    }
    let mut ret = 0;
    for i in 0..n {
        if ret >= n - 1 || ret > n / 2 {
            break;
        }
        let mut mp = std::collections::HashMap::new();
        for j in i + 1..n {
            let mut x = points[i as usize][0] - points[j as usize][0];
            let mut y = points[i as usize][1] - points[j as usize][1];
            if x == 0 {
                y = 1;
            } else if y == 0 {
                x = 1;
            } else {
                if y < 0 {
                    x = -x;
                    y = -y;
                }
                let gcd_xy = gcd(x.abs(), y.abs());
                x /= gcd_xy;
                y /= gcd_xy;
            }
            let count = mp.entry(y + x * 20001).or_insert(0);
            *count += 1;
        }
        let mut maxn = 0;

        for val in mp.values() {
            maxn = maxn.max(val + 1);
        }
        ret = ret.max(maxn);
    }
    return ret;
}

fn main() {
    {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(max_points(points), 3);
    }
    {
        let points = vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ];
        assert_eq!(max_points(points), 4);
    }
}
