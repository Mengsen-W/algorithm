struct Solution;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items.clone();
        // 将物品按价格升序排序
        items.sort_by(|a, b| a[0].cmp(&b[0]));
        let n = items.len();
        // 按定义修改美丽值
        for i in 1..n {
            items[i][1] = items[i][1].max(items[i - 1][1]);
        }
        // 二分查找处理查询
        queries
            .into_iter()
            .map(|q| Self::query(&items, q, n))
            .collect()
    }

    fn query(items: &Vec<Vec<i32>>, q: i32, n: usize) -> i32 {
        let mut l = 0;
        let mut r = n;
        while l < r {
            let mid = l + (r - l) / 2;
            if items[mid][0] > q {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        if l == 0 {
            // 此时所有物品价格均大于查询价格
            0
        } else {
            // 返回小于等于查询价格的物品的最大美丽值
            items[l - 1][1]
        }
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
            vec![1, 2, 3, 4, 5, 6],
            vec![2, 4, 5, 5, 6, 6],
        ),
        (
            vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]],
            vec![1],
            vec![4],
        ),
        (vec![vec![10, 1000]], vec![5], vec![0]),
    ];

    for (items, queries, ans) in tests {
        assert_eq!(Solution::maximum_beauty(items, queries), ans);
    }
}
