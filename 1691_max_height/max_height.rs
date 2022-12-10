/*
 * @Date: 2022-12-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-10
 * @FilePath: /algorithm/1691_max_height/max_height.rs
 */

pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
    let mut cuboids = cuboids;
    for c in cuboids.iter_mut() {
        c.sort_unstable();
    }
    cuboids.sort_unstable_by(|a, b| (a[0] + a[1] + a[2]).cmp(&(b[0] + b[1] + b[2])));

    let mut memo = vec![-1; cuboids.len()];

    fn check(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        a[0] <= b[0] && a[1] <= b[1] && a[2] <= b[2]
    }

    fn dfs(top: i32, index: usize, cuboids: &Vec<Vec<i32>>, memo: &mut Vec<i32>) -> i32 {
        if index == cuboids.len() {
            return 0;
        }
        if top != -1 && memo[top as usize] != -1 {
            return memo[top as usize];
        }
        let mut height = dfs(top, index + 1, cuboids, memo);
        if top == -1 || check(&cuboids[top as usize], &cuboids[index]) {
            height = std::cmp::max(
                height,
                cuboids[index][2] + dfs(index as i32, index + 1, cuboids, memo),
            );
        }
        if top != -1 {
            memo[top as usize] = height;
        }
        height
    }
    dfs(-1, 0, &cuboids, &mut memo)
}

fn main() {
    {
        let cuboids = vec![vec![50, 45, 20], vec![95, 37, 53], vec![45, 23, 12]];
        let ans = 190;
        assert_eq!(max_height(cuboids), ans);
    }

    {
        let cuboids = vec![vec![38, 25, 45], vec![76, 35, 3]];
        let ans = 76;
        assert_eq!(max_height(cuboids), ans);
    }

    {
        let cuboids = vec![
            vec![7, 11, 17],
            vec![7, 17, 11],
            vec![11, 7, 17],
            vec![11, 17, 7],
            vec![17, 7, 11],
            vec![17, 11, 7],
        ];
        let ans = 102;
        assert_eq!(max_height(cuboids), ans);
    }
}
