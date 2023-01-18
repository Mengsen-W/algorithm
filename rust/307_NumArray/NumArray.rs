/*
 * @Date: 2022-04-03 23:23:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-03 23:40:21
 * @FilePath: /algorithm/307_NumArray/NumArray.rs
 */

struct NumArray {
    s_tree: Vec<i32>,
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut new_array = Self {
            s_tree: vec![0; nums.len() + 1],
            nums: nums.clone(),
        };
        for (i, &num) in nums.iter().enumerate() {
            new_array.s_update(i as i32, num);
        }
        new_array
    }

    fn update(&mut self, i: i32, val: i32) {
        self.s_update(i, val - self.nums[i as usize]);
        self.nums[i as usize] = val;
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.s_sum(j + 1) - self.s_sum(i)
    }

    fn s_update(&mut self, i: i32, val: i32) {
        let mut i = i as usize + 1;
        let n = self.nums.len();
        while i <= n {
            self.s_tree[i] += val;
            i += i & (!i + 1);
        }
    }

    fn s_sum(&self, i: i32) -> i32 {
        let mut res = 0;
        let mut i = i as usize;
        while i > 0 {
            res += self.s_tree[i];
            i -= i & (!i + 1);
        }
        res
    }
}

fn main() {
    let mut n = NumArray::new(vec![1, 3, 5]);
    assert_eq!(n.sum_range(0, 2), 9);
    n.update(1, 2);
    assert_eq!(n.sum_range(0, 2), 8);
}
