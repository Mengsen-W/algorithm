/*
 * @Date: 2024-03-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-18
 * @FilePath: /algorithm/rust/303_NumArray/NumArray.rs
 */

struct NumArray {
    s: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut s = vec![0; nums.len() + 1];
        for (i, &x) in nums.iter().enumerate() {
            s[i + 1] = s[i] + x;
        }
        Self { s }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.s[right as usize + 1] - self.s[left as usize]
    }
}

fn main() {
    let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(num_array.sum_range(0, 2), 1);
    assert_eq!(num_array.sum_range(2, 5), -1);
    assert_eq!(num_array.sum_range(0, 5), -3);
}
