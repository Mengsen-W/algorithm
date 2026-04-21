/*
 * @Date: 2021-04-17 10:10:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-17 11:54:39
 */

fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let mut mp: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let len = nums.len() as i32;
    let get_id = |x: i32, w: i64| {
        let x = x as i64;
        // println!("{} {}", x, w);
        if x < 0 {
            (x + 1) / (w - 1)
        } else {
            x / w
        }
    };
    for i in 0..len {
        let x = nums[i as usize];
        let id = get_id(x, t.wrapping_add(1).into()) as i32;
        // println!("{},{},{}", id, x, t);
        if mp.contains_key(&id) {
            return true;
        }
        if mp.contains_key(&(id - 1)) && (x - mp.get(&(id - 1)).unwrap()).abs() <= t {
            return true;
        }
        if mp.contains_key(&(id + 1)) && (x - mp.get(&(id + 1)).unwrap()).abs() <= t {
            return true;
        }
        mp.insert(id, x);
        if i >= k {
            mp.remove(&(get_id(nums[(i - k) as usize], t as i64 + 1) as i32));
        }
    }
    false
}

fn contains_nearby_almost_duplicate_2(nums: Vec<i32>, k: i32, t: i32) -> bool {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            let i_j = if i >= j { i - j } else { j - i } as i32;
            if i_j <= k && (nums[i] as i64 - nums[j] as i64).abs() <= t as i64 {
                return true;
            }
        }
    }
    false
}

fn main() {
    {
        let nums = vec![1, 2, 3, 1];
        assert!(contains_nearby_almost_duplicate(nums, 3, 0))
    }
    {
        let nums = vec![1, 0, 1, 1];
        assert!(contains_nearby_almost_duplicate(nums, 1, 2))
    }
    {
        let nums = vec![1, 5, 9, 1, 5, 9];
        assert!(!contains_nearby_almost_duplicate(nums, 2, 3))
    }
    {
        let nums = vec![2147483647, -1, 2147483647];
        assert!(!contains_nearby_almost_duplicate_2(nums, 1, 2147483647))
    }
}
