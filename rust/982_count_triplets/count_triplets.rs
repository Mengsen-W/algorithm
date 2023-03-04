/*
 * @Date: 2023-03-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-04
 * @FilePath: /algorithm/rust/982_count_triplets/count_triplets.rs
 */

pub fn count_triplets(nums: Vec<i32>) -> i32 {
    let mut cnt = [0i16; 65536]; // u16 节约内存
    for i in 0..nums.len() {
        cnt[nums[i] as usize] += 1;
        for j in 0..i {
            cnt[nums[i] as usize & nums[j] as usize] += 2
        }
    }
    nums.into_iter()
        .map(|mut x| {
            x ^= 65535;
            if x == 0 {
                cnt[0] as i32
            } else {
                let mut j = x;
                let mut c = 0;
                loop {
                    c += cnt[j as usize] as i32;
                    j = (j - 1) & x;
                    if j == 0 {
                        break c + cnt[0] as i32;
                    }
                }
            }
        })
        .sum()
}

fn main() {
    {
        let nums = vec![2, 1, 3];
        let ans = 12;
        assert_eq!(count_triplets(nums), ans);
    }

    {
        let nums = vec![0, 0, 0];
        let ans = 27;
        assert_eq!(count_triplets(nums), ans);
    }
}
