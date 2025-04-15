struct Solution;

struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        FenwickTree {
            tree: vec![0; size + 1],
        }
    }

    fn update(&mut self, index: usize, delta: i32) {
        let mut idx = index + 1;
        while idx < self.tree.len() {
            self.tree[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, index: usize) -> i32 {
        let mut idx = index + 1;
        let mut res = 0;
        while idx > 0 {
            res += self.tree[idx];
            idx -= idx & (!idx + 1);
        }
        res
    }
}

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut pos2 = vec![0; n];
        for (i, &num) in nums2.iter().enumerate() {
            pos2[num as usize] = i;
        }
        let mut reversed_index_mapping = vec![0; n];
        for (i, &num) in nums1.iter().enumerate() {
            reversed_index_mapping[pos2[num as usize]] = i;
        }
        let mut tree = FenwickTree::new(n);
        let mut res = 0i64;
        for value in 0..n {
            let pos = reversed_index_mapping[value];
            let left = tree.query(pos) as i64;
            tree.update(pos, 1);
            let right = (n - 1 - pos) as i64 - (value as i64 - left);
            res += left * right;
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![2, 0, 1, 3], vec![0, 1, 2, 3], 1),
        (vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3], 4),
    ];

    for (nums1, nums2, expected) in tests {
        assert_eq!(Solution::good_triplets(nums1, nums2), expected);
    }
}
