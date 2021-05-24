/*
 * @Date: 2021-05-24 09:54:54
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-24 10:19:09
 */

#[derive(Default)]
pub struct Trie {
    root: Node,
}

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 2],
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, num: i32) {
        let mut node = &mut self.root;
        for i in 0..32 {
            let idx = ((num >> (31 - i)) & 1) as usize;
            let next = &mut node.children[idx];
            node = next.get_or_insert_with(Box::<Node>::default);
        }
    }
    pub fn get_max_xor_num(&self, num: i32) -> i32 {
        let mut node = &self.root;
        let mut res = [0; 32];
        for i in 0..32 {
            let mut idx = (((num >> (31 - i)) & 1) ^ 1) as usize;
            if node.children[idx].is_none() {
                idx = 1 ^ idx;
            }
            if let Some(next) = &node.children[idx] {
                node = next.as_ref();
                res[i] = idx;
            }
        }
        res.iter().fold(0, |acc, x| acc * 2 + *x as i32)
    }
}

pub fn maximize_xor(mut nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    nums.sort_unstable_by_key(|x| -x);
    for (index, query) in queries.iter_mut().enumerate() {
        query.push(index as i32);
    }
    queries.sort_unstable_by_key(|x| x[1]);
    let mut res = vec![0; queries.len()];
    let mut trie = Trie::new();
    for query in queries {
        let (x, m, pos) = (query[0], query[1], query[2]);
        while nums.len() > 0 && nums[nums.len() - 1] <= m {
            trie.insert(nums[nums.len() - 1]);
            nums.pop();
        }
        if nums.len() < n {
            res[pos as usize] = trie.get_max_xor_num(x) ^ x;
        } else {
            res[pos as usize] = -1;
        }
    }
    res
}

fn main() {
    {
        let nums = vec![0, 1, 2, 3, 4];
        let queries = vec![vec![3, 1], vec![1, 3], vec![5, 6]];
        assert_eq!(maximize_xor(nums, queries), vec![3, 3, 7]);
    }
    {
        let nums = vec![5, 2, 4, 6, 6, 3];
        let queries = vec![vec![12, 4], vec![8, 1], vec![6, 3]];
        assert_eq!(maximize_xor(nums, queries), vec![15, -1, 5]);
    }
}
