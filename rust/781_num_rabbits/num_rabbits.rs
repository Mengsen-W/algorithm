/*
 * @Date: 2021-04-04 19:46:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-04 19:51:09
 */

fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for i in answers {
        *map.entry(i).or_insert(0) += 1;
    }
    let mut res: i32 = 0;
    for (x, y) in map {
        res += (x + y) / (x + 1) * (x + 1);
    }
    res
}

fn main() {
    let nums = vec![1, 1, 2];
    assert_eq!(5, num_rabbits(nums));
    let nums = vec![10, 10, 10];
    assert_eq!(11, num_rabbits(nums));
    let nums = vec![];
    assert_eq!(0, num_rabbits(nums));
}
