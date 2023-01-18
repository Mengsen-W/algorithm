/*
 * @Date: 2022-10-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-15
 * @FilePath: /algorithm/1441_build_array/build_array.rs
 */

pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut res = Vec::new();
    let mut pos = 0;
    for i in 1..(n + 1) {
        if target[pos] == i {
            res.push("Push".to_string());
            pos += 1;
            if pos >= target.len() {
                break;
            }
        } else {
            res.push("Push".to_string());
            res.push("Pop".to_string());
        }
    }
    res
}

fn main() {
    {
        let target = vec![1, 3];
        let n = 3;
        let ans: Vec<String> = vec!["Push", "Push", "Pop", "Push"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(build_array(target, n), ans);
    }

    {
        let target = vec![1, 2, 3];
        let n = 3;
        let ans: Vec<String> = vec!["Push", "Push", "Push"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(build_array(target, n), ans)
    }

    {
        let target = vec![1, 2];
        let n = 4;
        let ans: Vec<String> = vec!["Push", "Push"].iter().map(|s| s.to_string()).collect();
        assert_eq!(build_array(target, n), ans)
    }
}
