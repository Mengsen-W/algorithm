/*
 * @Date: 2023-09-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-27
 * @FilePath: /algorithm/rust/1333_filter_restaurants/filter_restaurants.rs
 */

struct Solution;
impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut tmp = restaurants
            .iter()
            .filter(|&item| {
                item[2] >= vegan_friendly && item[3] <= max_price && item[4] <= max_distance
            })
            .map(|item| (item[0], item[1]))
            .collect::<Vec<(i32, i32)>>();

        tmp.sort_by(|(left_id, left_rating), (right_id, right_rating)| {
            if left_rating != right_rating {
                return right_rating.cmp(left_rating);
            }
            return right_id.cmp(left_id);
        });

        tmp.iter().map(|item| item.0).collect::<Vec<i32>>()
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![1, 4, 1, 40, 10],
                vec![2, 8, 0, 50, 5],
                vec![3, 8, 1, 30, 4],
                vec![4, 10, 0, 10, 3],
                vec![5, 1, 1, 15, 1],
            ],
            1,
            50,
            10,
            vec![3, 1, 5],
        ),
        (
            vec![
                vec![1, 4, 1, 40, 10],
                vec![2, 8, 0, 50, 5],
                vec![3, 8, 1, 30, 4],
                vec![4, 10, 0, 10, 3],
                vec![5, 1, 1, 15, 1],
            ],
            0,
            50,
            10,
            vec![4, 3, 2, 1, 5],
        ),
        (
            vec![
                vec![1, 4, 1, 40, 10],
                vec![2, 8, 0, 50, 5],
                vec![3, 8, 1, 30, 4],
                vec![4, 10, 0, 10, 3],
                vec![5, 1, 1, 15, 1],
            ],
            0,
            30,
            3,
            vec![4, 5],
        ),
    ];

    for (restaurants, vegan_friendly, max_price, max_distance, expected) in tests {
        assert_eq!(
            Solution::filter_restaurants(restaurants, vegan_friendly, max_price, max_distance),
            expected
        );
    }
}
