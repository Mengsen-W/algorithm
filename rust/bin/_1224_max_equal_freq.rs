/*
 * @Date: 2022-08-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-18
 * @FilePath: /algorithm/1224_max_equal_freq/max_equal_freq.rs
 */

pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
    let (mut cnt, mut freq) = (vec![0; 100001], vec![0; 100001]);
    nums.iter().for_each(|num| {
        cnt[*num as usize] += 1;
        freq[cnt[*num as usize] as usize] += 1;
    });
    for i in (1..nums.len()).rev() {
        if freq[cnt[nums[i] as usize] as usize] * cnt[nums[i] as usize] == i {
            return i as i32 + 1;
        }
        freq[cnt[nums[i] as usize] as usize] -= 1;
        cnt[nums[i] as usize] -= 1;
        if freq[cnt[nums[i - 1] as usize] as usize] * cnt[nums[i - 1] as usize] == i {
            return i as i32 + 1;
        }
    }
    1
}

fn main() {
    assert_eq!(max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]), 7);
    assert_eq!(
        max_equal_freq(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5]),
        13
    );
}
