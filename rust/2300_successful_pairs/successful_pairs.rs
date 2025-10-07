struct Solution;

impl Solution {
    pub fn successful_pairs(mut spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        let last = potions[potions.len() - 1] as i64;
        for x in spells.iter_mut() {
            let target = (success - 1) / *x as i64;
            if target < last {
                // 防止 i64 转成 i32 溢出（这样不需要把 potions 转成 i64 比较）
                let j = potions.partition_point(|&x| x <= target as i32);
                *x = (potions.len() - j) as i32;
            } else {
                *x = 0;
            }
        }
        spells
    }
}

fn main() {
    let tests = vec![
        (vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7, vec![4, 0, 3]),
        (vec![3, 1, 2], vec![8, 5, 8], 16, vec![2, 0, 2]),
    ];

    for (spells, potions, success, ans) in tests {
        assert_eq!(Solution::successful_pairs(spells, potions, success), ans);
    }
}
