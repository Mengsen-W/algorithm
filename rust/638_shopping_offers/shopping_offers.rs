/*
 * @Date: 2021-10-24 02:03:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-24 02:43:07
 */

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn shopping_offers(price: Vec<i32>, mut special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        cache.insert(vec![0; needs.len()], 0);

        special.retain(|v| {
            v.last().unwrap() < &(0..price.len()).fold(0, |acc, i| acc + price[i] * v[i])
        });

        Self::go(&mut cache, &price, special.iter().collect(), needs)
    }

    fn go(
        cache: &mut HashMap<Vec<i32>, i32>,
        price: &Vec<i32>,
        special: Vec<&Vec<i32>>,
        needs: Vec<i32>,
    ) -> i32 {
        if cache.contains_key(&needs) {
            *cache.get(&needs).unwrap()
        } else {
            let mut min_price = (0..needs.len()).fold(0, |acc, i| acc + needs[i] * price[i]);

            let relevant_specials = special
                .iter()
                .filter(|v| (0..needs.len()).all(|i| v[i] <= needs[i]))
                .cloned()
                .collect::<Vec<&Vec<i32>>>();

            for s in &relevant_specials {
                let new_needs = (0..needs.len())
                    .map(|i| needs[i] - s[i])
                    .collect::<Vec<i32>>();
                min_price = min_price.min(
                    s.last().unwrap()
                        + Self::go(cache, price, relevant_specials.clone(), new_needs),
                );
            }

            cache.insert(needs, min_price);

            min_price
        }
    }
}

fn main() {
    {
        let price = vec![2, 5];
        let special = vec![vec![3, 0, 5], vec![1, 2, 10]];
        let needs = vec![3, 2];
        assert_eq!(Solution::shopping_offers(price, special, needs), 14);
    }
    {
        let price = vec![2, 3, 4];
        let special = vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]];
        let needs = vec![1, 2, 1];
        assert_eq!(Solution::shopping_offers(price, special, needs), 11);
    }
}
