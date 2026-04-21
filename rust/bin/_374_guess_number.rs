/*
 * @Date: 2021-06-14 09:36:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-14 10:11:14
 */

static mut PICK: i32 = 0;

unsafe fn guess(n: i32) -> i32 {
    if n > PICK {
        -1
    } else if n < PICK {
        1
    } else {
        0
    }
}

fn guess_number(n: i32) -> i32 {
    let mut _left = 0;
    let mut _right = n;
    while _left <= _right {
        let mid = _left + ((_right - _left) / 2);
        let _res = unsafe { guess(mid) };
        if _res == 0 {
            return mid;
        } else if _res == 1 {
            _left = mid + 1;
        } else {
            _right = mid - 1;
        }
    }
    _left
}

fn main() {
    {
        unsafe { PICK = 6 };
        let n = 10;
        unsafe { assert_eq!(guess_number(n), PICK) }
    }
    {
        unsafe { PICK = 1 };
        let n = 1;
        unsafe { assert_eq!(guess_number(n), PICK) }
    }
    {
        unsafe { PICK = 1 };
        let n = 2;
        unsafe { assert_eq!(guess_number(n), PICK) }
    }
    {
        unsafe { PICK = 2 };
        let n = 2;
        unsafe { assert_eq!(guess_number(n), PICK) }
    }
}
