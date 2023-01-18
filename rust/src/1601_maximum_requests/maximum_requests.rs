/*
 * @Date: 2022-02-28 02:47:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-28 03:49:59
 * @FilePath: /algorithm/1601_maximum_requests/maximum_requests.rs
 */

pub fn maximum_requests1(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    fn dfs(
        delta: &mut Vec<i32>,
        requests: &Vec<Vec<i32>>,
        pos: usize,
        cnt: &mut i32,
        zero: &mut i32,
        ans: &mut i32,
        n: &i32,
    ) {
        if pos == requests.len() {
            if *zero == *n {
                *ans = std::cmp::max(*ans, *cnt);
            }
            return;
        }
        dfs(delta, requests, pos + 1, cnt, zero, ans, n);
        let z = *zero;
        *cnt += 1;
        let r = &requests[pos];
        let (x, y) = (r[0] as usize, r[1] as usize);
        *zero -= if delta[x] == 0 { 1 } else { 0 };
        delta[x] -= 1;
        *zero += if delta[x] == 0 { 1 } else { 0 };
        *zero -= if delta[y] == 0 { 1 } else { 0 };
        delta[y] += 1;
        *zero += if delta[y] == 0 { 1 } else { 0 };
        dfs(delta, requests, pos + 1, cnt, zero, ans, n);
        delta[y] -= 1;
        delta[x] += 1;
        *cnt -= 1;
        *zero = z;
    }

    let mut delta: Vec<i32> = vec![0; n as usize];
    let (mut ans, mut cnt, mut zero) = (0, 0, n);
    dfs(&mut delta, &requests, 0, &mut cnt, &mut zero, &mut ans, &n);
    ans
}

pub fn maximum_requests2(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    let mut delta: Vec<i32>;
    let (mut ans, m) = (0, requests.len());
    for mask in 0..(1_u32 << m) {
        let cnt = mask.count_ones();
        if cnt <= ans {
            continue;
        }
        delta = vec![0; n as usize];
        for i in 0..m {
            if mask & (1 << i) != 0 {
                delta[requests[i][0] as usize] += 1;
                delta[requests[i][1] as usize] -= 1;
            }
        }
        if delta.iter().all(|&x| x == 0) {
            ans = cnt;
        }
    }
    ans as i32
}

fn main() {
    {
        let input = vec![
            vec![0, 1],
            vec![1, 0],
            vec![0, 1],
            vec![1, 2],
            vec![2, 0],
            vec![3, 4],
        ];
        assert_eq!(maximum_requests1(5, input.clone()), 5);
        assert_eq!(maximum_requests2(5, input), 5);
    }

    {
        let input = vec![vec![0, 0], vec![1, 2], vec![2, 1]];
        assert_eq!(maximum_requests1(5, input.clone()), 3);
        assert_eq!(maximum_requests2(5, input), 3);
    }

    {
        let input = vec![vec![0, 3], vec![3, 1], vec![1, 2], vec![2, 0]];
        assert_eq!(maximum_requests1(5, input.clone()), 4);
        assert_eq!(maximum_requests2(5, input), 4);
    }
}
