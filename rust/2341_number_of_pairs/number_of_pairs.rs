/*
 * @Date: 2023-02-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-16
 * @FilePath: /algorithm/rust/2341_number_of_pairs/number_of_pairs.rs
 */

pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    let mut map = vec![0; 101];
    nums.into_iter().for_each(|x| map[x as usize] += 1);
    let mut ans = vec![0; 2];
    map.into_iter().for_each(|x| match x {
        0 => (),
        x => {
            ans[0] += x / 2;
            ans[1] += x & 1;
        }
    });
    ans
}

fn main() {
    {
        let nums = vec![1, 3, 2, 1, 3, 2, 2];
        let ans = vec![3, 1];
        assert_eq!(number_of_pairs(nums), ans);
    }

    {
        let nums = vec![1, 1];
        let ans = vec![1, 0];
        assert_eq!(number_of_pairs(nums), ans);
    }

    {
        let nums = vec![0];
        let ans = vec![0, 1];
        assert_eq!(number_of_pairs(nums), ans);
    }
}
