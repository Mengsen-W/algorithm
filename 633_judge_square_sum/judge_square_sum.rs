/*
 * @Date: 2021-04-28 09:33:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-28 10:03:54
 */

fn judge_square_sum(c: i32) -> bool {
    let c = c as i64;
    let mut left: i64 = 0;
    let mut right: i64 = ((c as f64).sqrt()) as i64;
    while left <= right {
        // println!("{} and {}", left, right);
        let sum: i64 = left.pow(2) + right.pow(2);
        if sum > c {
            right -= 1;
        } else if sum < c {
            left += 1;
        } else {
            return true;
        }
    }
    false
}

fn main() {
    {
        let c = 5;
        assert!(judge_square_sum(c));
    }
    {
        let c = 3;
        assert!(!judge_square_sum(c));
    }
    {
        let c = 4;
        assert!(judge_square_sum(c));
    }
    {
        let c = 2;
        assert!(judge_square_sum(c));
    }
    {
        let c = 1;
        assert!(judge_square_sum(c));
    }
    {
        let c = 1000;
        assert!(judge_square_sum(c));
    }
}
