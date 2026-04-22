struct Solution;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_dia_sq = 0;
        let mut max_area = 0;
        for dim in dimensions {
            let l = dim[0];
            let w = dim[1];
            let dia_sq = l * l + w * w;
            let area = l * w;
            if dia_sq > max_dia_sq {
                max_dia_sq = dia_sq;
                max_area = area;
            } else if dia_sq == max_dia_sq {
                max_area = std::cmp::max(max_area, area);
            }
        }
        max_area
    }
}

fn main() {
    let tests = vec![
        (vec![vec![9, 3], vec![8, 6]], 48),
        (vec![vec![3, 4], vec![4, 3]], 12),
    ];

    for (dimensions, expected) in tests {
        assert_eq!(Solution::area_of_max_diagonal(dimensions), expected);
    }
}
