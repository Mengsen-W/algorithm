/*
 * @Date: 2023-07-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-26
 * @FilePath: /algorithm/rust/2569_handle_query/handle_query.rs
 */

struct Solution;

struct SegmentTree {
    payload: Vec<i32>,
    lazy: Vec<u8>,
    src_len: usize,
}

impl SegmentTree {
    fn build_tree(&mut self, left: usize, right: usize, pos: usize, source: &Vec<i32>) {
        if left == right {
            self.payload[pos] = source[left];
            return;
        }
        let mid = left + (right - left) / 2;
        let lpos = pos * 2 + 1;
        let rpos = pos * 2 + 2;
        self.build_tree(left, mid, lpos, source);
        self.build_tree(mid + 1, right, rpos, source);

        self.payload[pos] = self.payload[lpos] + self.payload[rpos];
    }

    pub fn new(source: &Vec<i32>) -> SegmentTree {
        let mut seg_tree = SegmentTree {
            payload: vec![0; source.len() * 4],
            lazy: vec![0; source.len() * 4],
            src_len: source.len(),
        };
        seg_tree.build_tree(0, source.len() - 1, 0, source);
        seg_tree
    }

    pub fn sum(&self) -> i32 {
        self.payload[0]
    }

    fn flip_range_impl(&mut self, range: (usize, usize), l: usize, r: usize, p: usize) {
        if range.0 <= l && range.1 >= r {
            self.payload[p] = (r - l + 1) as i32 - self.payload[p];
            if l != r {
                self.lazy[p] ^= 1;
            }
            return;
        }

        let m = l + (r - l) / 2;
        let lp = p * 2 + 1;
        let rp = p * 2 + 2;

        if self.lazy[p] == 1 {
            self.payload[lp] = (m - l + 1) as i32 - self.payload[lp];
            self.payload[rp] = (r - m) as i32 - self.payload[rp];
            self.lazy[lp] ^= 1;
            self.lazy[rp] ^= 1;
            self.lazy[p] = 0;
        }

        if range.0 <= m {
            self.flip_range_impl(range, l, m, lp);
        }
        if range.1 > m {
            self.flip_range_impl(range, m + 1, r, rp);
        }

        self.payload[p] = self.payload[lp] + self.payload[rp];
    }

    pub fn flip_range(&mut self, rangel: usize, ranger: usize) {
        self.flip_range_impl((rangel, ranger), 0, self.src_len - 1, 0);
    }
}

impl Solution {
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let mut nums2_sum: i64 = nums2.iter().map(|&x| x as i64).sum();
        let mut res = Vec::new();

        let mut seg_tree = SegmentTree::new(&nums1);
        for query in queries.iter() {
            match query[0] {
                1 => {
                    seg_tree.flip_range(query[1] as usize, query[2] as usize);
                }
                2 => {
                    nums2_sum += query[1] as i64 * seg_tree.sum() as i64;
                }
                3 => {
                    res.push(nums2_sum);
                }
                _ => unreachable!(),
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![vec![1, 1, 1], vec![2, 1, 0], vec![3, 0, 0]],
            vec![3],
        ),
        (
            vec![1],
            vec![5],
            vec![vec![2, 0, 0], vec![3, 0, 0]],
            vec![5],
        ),
    ];

    for (nums1, nums2, queries, res) in tests {
        assert_eq!(Solution::handle_query(nums1, nums2, queries), res)
    }
}
