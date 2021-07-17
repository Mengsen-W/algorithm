/*
 * @Date: 2021-07-17 16:15:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-17 16:53:28
 */

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        struct Status {
            l_sum: i32,
            r_sum: i32,
            m_sum: i32,
            i_sum: i32,
        }
        fn push_up(l: Status, r: Status) -> Status {
            let i_sum = l.i_sum + r.i_sum;
            let l_sum = l.l_sum.max(l.i_sum + r.l_sum);
            let r_sum = r.r_sum.max(r.i_sum + l.r_sum);
            let m_sum = (l.m_sum.max(r.m_sum)).max(l.r_sum + r.l_sum);
            Status {
                l_sum: l_sum,
                r_sum: r_sum,
                m_sum: m_sum,
                i_sum: i_sum,
            }
        }

        fn get(a: &Vec<i32>, l: i32, r: i32) -> Status {
            if l == r {
                let l = l as usize;
                return Status {
                    l_sum: a[l],
                    r_sum: a[l],
                    m_sum: a[l],
                    i_sum: a[l],
                };
            }
            let m = (l + r) >> 1;
            let l_sub: Status = get(a, l, m);
            let r_sub: Status = get(a, m + 1, r);
            push_up(l_sub, r_sub)
        }

        get(&nums, 0, nums.len() as i32 - 1).m_sum
    }
}

fn main() {
    let nums = vec![1, 2, -1, -2, 2, 1, -2, 1, 4, -5, 4];
    assert_eq!(Solution::max_sub_array(nums), 6);
}
