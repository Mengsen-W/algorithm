/*
 * @Date: 2022-09-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-16
 * @FilePath: /algorithm/850_rectangle_area/rectangle_area.rs
 */

struct Solution;

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut set = std::collections::HashSet::with_capacity(rectangles.len());
        rectangles.iter().for_each(|t| {
            set.insert(t[1]); // 下边界
            set.insert(t[3]); // 上边界
        });
        let mut hbound: Vec<_> = set.into_iter().collect();
        hbound.sort_unstable();
        let mut tree = SegTree::new(&hbound);
        let mut sweep = Vec::with_capacity(rectangles.len() << 1);
        rectangles.iter().enumerate().for_each(|(i, t)| {
            sweep.push((t[0], i, 1)); // 左边界
            sweep.push((t[2], i, -1)); // 右边界
        });
        sweep.sort_unstable();
        let mut ans: i64 = 0;
        let mut i = 0;
        while i < sweep.len() {
            let mut j = i;
            while j + 1 < sweep.len() && sweep[i].0 == sweep[j + 1].0 {
                j += 1;
            }
            if j + 1 == sweep.len() {
                break;
            }
            // 一次性地处理掉一批横坐标相同的左右边界
            for t in &sweep[i..=j] {
                // 使用二分查找得到完整覆盖的线段的编号范围
                let left = hbound.binary_search(&rectangles[t.1][1]).unwrap() + 1;
                let right = hbound.binary_search(&rectangles[t.1][3]).unwrap();
                tree.update(1, 1, hbound.len() - 1, left, right, t.2);
            }
            ans += tree.length() * (sweep[j + 1].0 - sweep[j].0) as i64;
            i = j + 1;
        }
        (ans % 1000000007) as i32
    }
}

#[derive(Clone, Copy)]
struct TreeNode {
    cover: i32,
    len: i32,
    max_len: i32,
}

struct SegTree {
    tree: Vec<TreeNode>,
}

impl SegTree {
    fn length(&self) -> i64 {
        self.tree[1].len as i64
    }

    fn new(hbound: &Vec<i32>) -> Self {
        fn init(
            tree: &mut Vec<TreeNode>,
            hbound: &Vec<i32>,
            idx: usize,
            left: usize,
            right: usize,
        ) {
            if left == right {
                tree[idx].max_len = hbound[left] - hbound[left - 1];
                return;
            }
            let mid = (left + right) >> 1;
            init(tree, hbound, idx * 2, left, mid);
            init(tree, hbound, idx * 2 + 1, mid + 1, right);
            tree[idx].max_len = tree[idx * 2].max_len + tree[idx * 2 + 1].max_len;
        }
        let len = hbound.len();
        let mut tree = vec![
            TreeNode {
                cover: 0,
                len: 0,
                max_len: 0,
            };
            len * 4 + 1
        ];
        init(&mut tree, hbound, 1, 1, len - 1);
        Self { tree }
    }

    fn push_up(&mut self, idx: usize, left: usize, right: usize) {
        if self.tree[idx].cover > 0 {
            self.tree[idx].len = self.tree[idx].max_len;
        } else if left == right {
            self.tree[idx].len = 0;
        } else {
            self.tree[idx].len = self.tree[idx * 2].len + self.tree[idx * 2 + 1].len;
        }
    }

    fn update(
        &mut self,
        idx: usize,
        left: usize,
        right: usize,
        uleft: usize,
        uright: usize,
        diff: i32,
    ) {
        if left > uright || right < uleft {
            return;
        }
        if uleft <= left && right <= uright {
            self.tree[idx].cover += diff;
            self.push_up(idx, left, right);
            return;
        }
        let mid = (left + right) >> 1;
        self.update(idx * 2, left, mid, uleft, uright, diff);
        self.update(idx * 2 + 1, mid + 1, right, uleft, uright, diff);
        self.push_up(idx, left, right);
    }
}

fn main() {
    {
        let rectangles = vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]];
        let ans = 6;
        assert_eq!(Solution::rectangle_area(rectangles), ans);
    }

    {
        let rectangles = vec![vec![0, 0, 1000000000, 1000000000]];
        let ans = 49;
        assert_eq!(Solution::rectangle_area(rectangles), ans);
    }
}
