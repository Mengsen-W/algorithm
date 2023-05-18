/*
 * @Date: 2023-05-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-18
 * @FilePath: /algorithm/rust/1073_add_negabinary/add_negabinary.rs
 */

pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut iter1 = arr1.into_iter().rev();
    let mut iter2 = arr2.into_iter().rev();

    let mut carry = 0;
    let mut ret = vec![];

    loop {
        let d1 = iter1.next();
        let d2 = iter2.next();

        // Nothing to add.
        if d1.is_none() && d2.is_none() && carry == 0 {
            break;
        }

        let mut result = d1.unwrap_or(0) + d2.unwrap_or(0) + carry;
        match result {
            -1 => {
                result = 1;
                carry = 1;
            }
            2 | 3 => {
                result -= 2;
                carry = -1;
            }
            0 | 1 => carry = 0,
            _ => unreachable!(),
        }
        ret.push(result);
    }

    while let [_, .., 0] = ret[..] {
        ret.pop();
    }

    ret.reverse();
    ret
}

fn main() {
    {
        let arr1 = vec![1, 1, 1, 1, 1];
        let arr2 = vec![1, 0, 1];
        let ans = vec![1, 0, 0, 0, 0];
        assert_eq!(add_negabinary(arr1, arr2), ans)
    }

    {
        let arr1 = vec![0];
        let arr2 = vec![0];
        let ans = vec![0];
        assert_eq!(add_negabinary(arr1, arr2), ans)
    }

    {
        let arr1 = vec![0];
        let arr2 = vec![1];
        let ans = vec![1];
        assert_eq!(add_negabinary(arr1, arr2), ans)
    }
}
