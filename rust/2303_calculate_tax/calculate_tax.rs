/*
 * @Date: 2023-01-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-23
 * @FilePath: /algorithm/rust/2303_calculate_tax/calculate_tax.rs
 */

pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
    let mut total_tax: f64 = 0.0;
    let mut lower = 0;
    for bracket in brackets {
        let (upper, percent) = (bracket[0], bracket[1]);
        let tax = (income.min(upper) - lower) * percent;
        total_tax += tax as f64;
        if income <= upper {
            break;
        }
        lower = upper;
    }
    total_tax / 100.0
}

fn main() {
    {
        let brackets = [[3, 50], [7, 10], [12, 25]]
            .iter()
            .map(|s| s.to_vec())
            .collect();
        let income = 10;
        let ans = 2.65000000;
        assert_eq!(calculate_tax(brackets, income), ans);
    }

    {
        let brackets = [[1, 0], [4, 25], [5, 50]]
            .iter()
            .map(|s| s.to_vec())
            .collect();
        let income = 2;
        let ans = 0.25000000;
        assert_eq!(calculate_tax(brackets, income), ans);
    }

    {
        let brackets = [[2, 50]].iter().map(|s| s.to_vec()).collect();
        let income = 0;
        let ans = 0.000000;
        assert_eq!(calculate_tax(brackets, income), ans);
    }
}
