/*
 * @Date: 2022-10-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-17
 * @FilePath: /algorithm/904_total_fruit/total_fruit.rs
 */

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    fruits
        .iter()
        .skip(1)
        .fold(
            (1, 1, 1, fruits[0], fruits[0]),
            |(ans, cnt, c_cnt, last_fruit, other_fruit), &fruit| match (
                fruit == last_fruit,
                fruit == other_fruit,
            ) {
                (true, _) => (
                    ans.max(cnt + 1),
                    cnt + 1,
                    c_cnt + 1,
                    last_fruit,
                    other_fruit,
                ),
                (false, true) => (ans.max(cnt + 1), cnt + 1, 1, other_fruit, last_fruit),
                (false, false) => (ans.max(c_cnt + 1), c_cnt + 1, 1, fruit, last_fruit),
            },
        )
        .0
}

fn main() {
    {
        let fruits = vec![1, 2, 1];
        let ans = 3;
        assert_eq!(total_fruit(fruits), ans);
    }

    {
        let fruits = vec![0, 1, 2, 2];
        let ans = 3;
        assert_eq!(total_fruit(fruits), ans);
    }

    {
        let fruits = vec![1, 2, 3, 2, 2];
        let ans = 4;
        assert_eq!(total_fruit(fruits), ans);
    }

    {
        let fruits = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
        let ans = 5;
        assert_eq!(total_fruit(fruits), ans);
    }
}
