/*
 * @Date: 2021-06-11 08:30:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-11 08:52:39
 */

fn num_squares(n: i32) -> i32 {
    // 判断是否为完全平方数
    fn is_perfect_square(x: i32) -> bool {
        let y: i32 = ((x as f32).sqrt()) as i32;
        y * y == x
    }

    // 判断是否嫩表示为 4^k*(8m+7)
    fn check_answer4(mut x: i32) -> bool {
        while x % 4 == 0 {
            x /= 4;
        }
        x % 8 == 7
    }
    if is_perfect_square(n) {
        return 1;
    }

    if check_answer4(n) {
        return 4;
    }

    let mut _i = 1;
    while _i * _i <= n {
        let j = n - _i * _i;
        if is_perfect_square(j) {
            return 2;
        }
        _i += 1;
    }
    return 3;
}

fn main() {
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
}
