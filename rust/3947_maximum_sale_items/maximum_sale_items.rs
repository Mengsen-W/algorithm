struct Solution;

impl Solution {
    pub fn maximum_sale_items(mut items: Vec<Vec<i32>>, mut budget: i32) -> i32 {
        items.sort_unstable_by_key(|item| item[1]);
        let mx = items.iter().map(|item| item[0] as usize).max().unwrap_or(0);
        let mut freq = vec![0i32; mx + 1];
        for item in &items {
            freq[item[0] as usize] += 1;
        }
        for i in 1..=mx {
            if freq[i] > 0 {
                for j in (i * 2..=mx).step_by(i) {
                    freq[i] += freq[j];
                }
            }
        }

        let mn = items[0][1];
        let mut ans = 0;
        for item in &items {
            let (factor, price) = (item[0] as usize, item[1]);
            if price - mn > mn {
                break;
            }
            let cnt = freq[factor] - 1;
            let take = cnt.min(budget / price);
            ans += take * 2;
            budget -= take * price;
        }
        ans += budget / mn;
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 6], vec![2, 4], vec![3, 5]], 19, 5),
        (
            vec![
                vec![2, 8],
                vec![1, 10],
                vec![6, 6],
                vec![4, 12],
                vec![5, 20],
                vec![5, 17],
            ],
            35,
            7,
        ),
    ];

    for (item, budget, ans) in tests {
        assert_eq!(Solution::maximum_sale_items(item, budget), ans);
    }
}
