/*
 * @Date: 2023-02-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-28
 * @FilePath: /algorithm/rust/2363_merge_similar_items/merge_similar_items.rs
 */

pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut cnt = [0; 1001];
    for x in items1.iter() {
        cnt[x[0] as usize] += x[1];
    }
    for x in items2.iter() {
        cnt[x[0] as usize] += x[1];
    }
    cnt.iter()
        .enumerate()
        .filter_map(|(i, &v)| {
            if v == 0 {
                return None;
            }
            Some(vec![i as i32, v])
        })
        .collect()
}

fn main() {
    {
        let items1 = vec![[1, 1], [4, 5], [3, 8]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let items2 = vec![[3, 1], [1, 5]].iter().map(|v| v.to_vec()).collect();
        let ans: Vec<Vec<i32>> = vec![[1, 6], [3, 9], [4, 5]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(merge_similar_items(items1, items2), ans);
    }

    {
        let items1 = vec![[1, 1], [3, 2], [2, 3]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let items2 = vec![[2, 1], [3, 2], [1, 3]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans: Vec<Vec<i32>> = vec![[1, 4], [2, 4], [3, 4]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(merge_similar_items(items1, items2), ans);
    }

    {
        let items1 = vec![[1, 3], [2, 2]].iter().map(|v| v.to_vec()).collect();
        let items2 = vec![[7, 1], [2, 2], [1, 4]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans: Vec<Vec<i32>> = vec![[1, 7], [2, 4], [7, 1]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(merge_similar_items(items1, items2), ans);
    }
}
