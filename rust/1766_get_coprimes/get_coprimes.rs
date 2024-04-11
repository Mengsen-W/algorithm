/*
 * @Date: 2024-04-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-11
 * @FilePath: /algorithm/rust/1766_get_coprimes/get_coprimes.rs
 */

struct Solution;

impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut gcds = vec![vec![]; 51];
        let mut tmp = vec![vec![]; 51];
        let mut ans = vec![-1; n];
        let mut dep = vec![-1; n];
        let mut g = vec![vec![]; n];

        fn gcd(a: i32, b: i32) -> i32 {
            let mut a = a;
            let mut b = b;
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a.abs()
        }

        fn dfs(
            x: usize,
            depth: i32,
            dep: &mut [i32],
            ans: &mut [i32],
            nums: &Vec<i32>,
            tmp: &mut Vec<Vec<usize>>,
            gcds: &Vec<Vec<i32>>,
            g: &Vec<Vec<usize>>,
        ) {
            dep[x] = depth;
            for &val in &gcds[nums[x] as usize] {
                if tmp[val as usize].is_empty() {
                    continue;
                }
                let las = *tmp[val as usize].last().unwrap();
                if ans[x] == -1 || dep[las] > dep[ans[x] as usize] {
                    ans[x] = las as i32;
                }
            }
            tmp[nums[x] as usize].push(x);
            for &val in &g[x] {
                if dep[val] == -1 {
                    // 被访问过的点dep不为-1
                    dfs(val, depth + 1, dep, ans, nums, tmp, gcds, g);
                }
            }
            tmp[nums[x] as usize].pop();
        }

        // 初始化
        for i in 1..=50 {
            for j in 1..=50 {
                if gcd(i, j) == 1 {
                    gcds[i as usize].push(j);
                }
            }
        }
        for edge in &edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            g[x].push(y);
            g[y].push(x);
        }
        dfs(0, 1, &mut dep, &mut ans, &nums, &mut tmp, &gcds, &g);
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![2, 3, 3, 2],
            vec![vec![0, 1], vec![1, 2], vec![1, 3]],
            vec![-1, 0, 0, 1],
        ),
        (
            vec![5, 6, 10, 2, 3, 6, 15],
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
            ],
            vec![-1, 0, -1, 0, 0, 0, -1],
        ),
    ];

    for (nums, edges, ans) in tests {
        assert_eq!(Solution::get_coprimes(nums, edges), ans);
    }
}
