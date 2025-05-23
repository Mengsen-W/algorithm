struct Solution;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        fn mul_matrix(m1: &Vec<Vec<i64>>, m2: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
            static M: i64 = 1000000000 + 7;
            let (n1, n2, n3) = (m1.len(), m2.len(), m2[0].len());
            let mut res = vec![vec![0; n3]; n1];
            for i in 0..n1 {
                for k in 0..n3 {
                    for j in 0..n2 {
                        res[i][k] = (res[i][k] + m1[i][j] * m2[j][k]) % M;
                    }
                }
            }
            res
        }

        let mut n = n;
        let mut mat = vec![
            vec![0, 0, 0, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 0],
            vec![1, 1, 1, 1],
        ];
        let mut matn = vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];

        while n != 0 {
            if (n & 1) != 0 {
                matn = mul_matrix(&matn, &mat);
            }
            mat = mul_matrix(&mat, &mat);
            n >>= 1;
        }
        matn[3][3] as i32
    }
}

fn main() {
    let tests = vec![(1, 1), (3, 5), (30, 312342182)];

    for (n, ans) in tests {
        assert_eq!(Solution::num_tilings(n), ans);
    }
}
