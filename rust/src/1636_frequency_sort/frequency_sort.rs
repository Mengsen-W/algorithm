/*
 * @Date: 2022-09-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-19
 * @FilePath: /algorithm/1636_frequency_sort/frequency_sort.rs
 */

pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut n = vec![0; 201];
    nums.iter().for_each(|&x| {
        n[(x + 100) as usize] += 1;
    });
    let mut f = vec![];
    for (i, &n) in n.iter().enumerate() {
        if n > 0 {
            f.push((n as usize, i as i32 - 100));
        }
    }
    f.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    let mut ans = vec![];
    for (n, x) in f {
        ans = [ans, vec![x; n]].concat();
    }
    ans
}

fn main() {
    {
        let nums = vec![1, 1, 2, 2, 2, 3];
        let ans = vec![3, 1, 1, 2, 2, 2];
        assert_eq!(frequency_sort(nums), ans);
    }

    {
        let nums = vec![2, 3, 1, 3, 2];
        let ans = vec![1, 3, 3, 2, 2];
        assert_eq!(frequency_sort(nums), ans);
    }

    {
        let nums = vec![-1, 1, -6, 4, 5, -6, 1, 4, 1];
        let ans = vec![5, -1, 4, 4, -6, -6, 1, 1, 1];
        assert_eq!(frequency_sort(nums), ans);
    }
}
