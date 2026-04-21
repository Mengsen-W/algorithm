/*
 * @Date: 2022-12-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-17
 * @FilePath: /algorithm/1764_can_choose/can_choose.rs
 */

pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
    fn find(nums: &Vec<i32>, k: usize, g: &Vec<i32>) -> Option<usize> {
        let (m, n) = (g.len(), nums.len());
        if k + m > n {
            return None;
        }
        let mut pi = vec![0; m];

        let mut j = 0;
        for i in 1..m {
            while j > 0 && g[i] != g[j] {
                j = pi[j - 1];
            }
            if g[i] == g[j] {
                j += 1;
            }
            pi[i] = j;
        }

        let mut j = 0;
        for i in k..n {
            while j > 0 && nums[i] != g[j] {
                j = pi[j - 1];
            }
            if nums[i] == g[j] {
                j += 1;
            }
            if j == m {
                return Some(i - m + 1);
            }
        }
        return None;
    }

    let mut k = 0;
    for i in 0..groups.len() {
        match find(&nums, k as usize, &groups[i]) {
            None => return false,
            Some(a) => {
                k = a;
                k += groups[i].len();
            }
        }
    }
    return true;
}

fn main() {
    {
        let groups = vec![vec![1, -1, -1], vec![3, -2, 0]];
        let nums = vec![1, -1, 0, 1, -1, -1, 3, -2, 0];
        assert!(can_choose(groups, nums));
    }

    {
        let groups = vec![vec![10, -2], vec![1, 2, 3, 4]];
        let nums = vec![1, 2, 3, 4, 10, -2];
        assert!(!can_choose(groups, nums));
    }

    {
        let groups = vec![vec![1, 2, 3], vec![3, 4]];
        let nums = vec![7, 7, 1, 2, 3, 4, 7, 7];
        assert!(!can_choose(groups, nums));
    }
}
