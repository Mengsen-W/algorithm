/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-01 20:29:42
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-01 20:40:26
 */

struct NumArray {
    pre_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut pre = 0;
        let pre_sum: Vec<i32> = nums
            .iter()
            .map(|x| {
                pre += x;
                pre
            })
            .collect();

        NumArray { pre_sum: pre_sum }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        if i > j {
            return 0;
        }
        if i == 0 {
            return self.pre_sum[j as usize];
        }

        return self.pre_sum[j as usize] - self.pre_sum[i as usize - 1];
    }
}

fn main() {
    let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(obj.sum_range(0, 2), 1);
    assert_eq!(obj.sum_range(2, 5), -1);
    assert_eq!(obj.sum_range(0, 5), -3);
}
