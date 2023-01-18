/*
 * @Date: 2022-09-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-26
 * @FilePath: /algorithm/17.19_missing_two/missing_two.rs
 */

pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
    let mut xorsum = 0;
    let n = nums.len() as i32 + 2;
    for num in &nums {
        xorsum ^= num;
    }
    for i in 1..=n {
        xorsum ^= i;
    }
    // 防止溢出
    let lsb = if xorsum == i32::MIN {
        xorsum
    } else {
        xorsum & (-xorsum)
    };
    let (mut type1, mut type2) = (0, 0);
    for num in &nums {
        if num & lsb != 0 {
            type1 ^= num;
        } else {
            type2 ^= num;
        }
    }
    for i in 1..=n {
        if i & lsb != 0 {
            type1 ^= i;
        } else {
            type2 ^= i;
        }
    }
    return vec![type1, type2];
}

fn main() {
    {
        let nums = vec![1];
        let ans = vec![3, 2];
        assert_eq!(missing_two(nums), ans);
    }

    {
        let nums = vec![2, 3];
        let ans = vec![1, 4];
        assert_eq!(missing_two(nums), ans);
    }
}
