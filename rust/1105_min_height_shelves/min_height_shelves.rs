/*
 * @Date: 2023-04-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-23
 * @FilePath: /algorithm/rust/1105_min_height_shelves/min_height_shelves.rs
 */

pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let n = books.len();
    let mut dp = vec![1000000; n + 1];
    dp[0] = 0;

    for i in 0..n {
        let mut height = 0;
        let mut width = 0;

        for j in (0..=i).rev() {
            width += books[j][0];

            if width > shelf_width {
                break;
            }

            height = height.max(books[j][1]);
            dp[i + 1] = (dp[i + 1]).min(dp[j] + height);
        }
    }

    dp[n]
}

fn main() {
    {
        let books = vec![
            vec![1, 1],
            vec![2, 3],
            vec![2, 3],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 2],
        ];
        let shelf_width = 4;
        let ans = 6;
        assert_eq!(min_height_shelves(books, shelf_width), ans);
    }

    {
        let books = vec![vec![1, 3], vec![2, 4], vec![3, 2]];
        let shelf_width = 6;
        let ans = 4;
        assert_eq!(min_height_shelves(books, shelf_width), ans);
    }
}
