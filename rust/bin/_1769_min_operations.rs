/*
 * @Date: 2022-12-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-02
 * @FilePath: /algorithm/1769_min_operations/min_operations.rs
 */

pub fn min_operations(boxes: String) -> Vec<i32> {
    let boxes: Vec<i32> = boxes.bytes().map(|x| (x - b'0') as i32).collect();
    let mut cur = boxes.iter().enumerate().map(|(i, x)| (i as i32) * x).sum();
    let total: i32 = boxes.iter().filter(|&&x| x == 1).sum();
    let mut ans: Vec<i32> = Vec::with_capacity(boxes.len());
    let mut count = 0;
    for x in boxes.into_iter() {
        ans.push(cur);
        if x == 1 {
            count += 1;
        }
        cur += 2 * count - total; // cur + count - (total-count)
    }
    return ans;
}

fn main() {
    {
        let boxes = String::from("110");
        let ans = vec![1, 1, 3];
        assert_eq!(min_operations(boxes), ans);
    }

    {
        let boxes = String::from("001011");
        let ans = vec![11, 8, 5, 4, 3, 4];
        assert_eq!(min_operations(boxes), ans);
    }
}
