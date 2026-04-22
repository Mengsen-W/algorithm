/*
 * @Date: 2021-03-16 09:33:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-16 17:28:08
 */

fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let dx: Vec<i32> = vec![0, 1, 0, -1];
    let dy: Vec<i32> = vec![1, 0, -1, 0];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut d = 0;
    let mut ans: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];

    for i in 1..=n * n {
        ans[x as usize][y as usize] = i;
        let mut tx = x + dx[d];
        let mut ty = y + dy[d];
        if tx < 0 || tx >= n || ty < 0 || ty >= n || ans[tx as usize][ty as usize] != 0 {
            d = (d + 1) % 4;
            tx = x + dx[d];
            ty = y + dy[d];
        }

        x = tx;
        y = ty;
    }

    return ans;
}

fn main() {
    assert_eq!(
        generate_matrix(3),
        vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
    );
    assert_eq!(generate_matrix(1), vec![vec![1]]);
}
