struct Solution;

impl Solution {
    pub fn min_cost(
        start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
        let r1 = start_pos[0] as usize;
        let c1 = start_pos[1] as usize;
        let r2 = home_pos[0] as usize;
        let c2 = home_pos[1] as usize;
        let mut res = 0; // 总代价

        // 移动至家所在行，判断行间移动方向并计算对应代价
        if r2 >= r1 {
            for i in (r1 + 1)..=r2 {
                res += row_costs[i];
            }
        } else {
            for i in r2..r1 {
                res += row_costs[i];
            }
        }

        // 移动至家所在位置，判断列间移动方向并计算对应代价
        if c2 >= c1 {
            for i in (c1 + 1)..=c2 {
                res += col_costs[i];
            }
        } else {
            for i in c2..c1 {
                res += col_costs[i];
            }
        }

        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 0], vec![2, 3], vec![5, 4, 3], vec![8, 2, 6, 7], 18),
        (vec![0, 0], vec![0, 0], vec![5], vec![26], 0),
    ];

    for (start_pos, home_pos, row_costs, col_costs, expected) in tests {
        assert_eq!(
            Solution::min_cost(start_pos, home_pos, row_costs, col_costs),
            expected
        );
    }
}
