/*
 * @Date: 2023-05-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-02
 * @FilePath: /algorithm/rust/970_powerful_integers/powerful_integers.rs
 */

pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
    use std::collections::HashSet;
    if bound < 2 {
        return vec![];
    }
    let mut set = HashSet::new();
    let mut temp = 2;
    let mut i = 0;
    let mut j = 0;
    while temp <= bound {
        while temp <= bound {
            set.insert(temp);
            temp = x.pow(i) + y.pow(j);
            j += 1;
            if y == 1 && j > 1 {
                break;
            }
        }
        i += 1;
        j = 0;
        temp = x.pow(i) + y.pow(j);
        if x == 1 && i > 1 {
            break;
        }
    }
    let res = set.iter().map(|x| *x).collect();
    res
}

fn main() {
    use std::collections::HashSet;

    {
        let x = 2;
        let y = 3;
        let bound = 10;
        let ans = vec![2, 4, 10, 5, 3, 7, 9];
        assert_eq!(powerful_integers(x, y, bound), ans);
    }

    {
        let x = 3;
        let y = 5;
        let bound = 15;
        let ans = HashSet::from([2, 4, 6, 8, 10, 14]);
        assert_eq!(powerful_integers(x, y, bound), ans);
    }
}
