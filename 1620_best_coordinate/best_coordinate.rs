/*
 * @Date: 2022-11-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-02
 * @FilePath: /algorithm/1620_best_coordinate/best_coordinate.rs
 */

pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
    fn get_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
        ((a.0 - b.0).abs()).pow(2) + (a.1 - b.1).pow(2)
    }
    let (range, mut mx) = (51, 0);
    let mut temp: Vec<Vec<i32>> = Vec::new();
    let mut ans: Vec<Vec<i32>> = Vec::new();
    for i in 0..range {
        for j in 0..range {
            let point = vec![i, j];
            temp.push(point);
        }
    }
    for v1 in temp {
        let mut v1_max = 0;
        for v2 in &towers {
            let dist = get_dist((v1[0], v1[1]), (v2[0], v2[1]));
            if dist <= radius.pow(2) {
                v1_max += (v2[2] as f32 / (1f32 + (dist as f32).sqrt())) as i32;
            }
        }
        if v1_max >= mx {
            if v1_max > mx {
                ans.clear();
            }
            let t_vec = vec![v1[0], v1[1]];
            ans.push(t_vec);
            mx = v1_max;
        }
    }
    ans.sort_by(|a, b| {
        if a[0] != b[0] {
            a[0].cmp(&b[0])
        } else {
            a[1].cmp(&b[1])
        }
    });
    if mx == 0 {
        return vec![0; 2];
    }
    ans[0].clone()
}

fn main() {
    {
        let towers = vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]];
        let radius = 2;
        let ans = vec![2, 1];
        assert_eq!(best_coordinate(towers, radius), ans);
    }

    {
        let towers = vec![vec![23, 11, 21]];
        let radius = 9;
        let ans = vec![23, 11];
        assert_eq!(best_coordinate(towers, radius), ans);
    }

    {
        let towers = vec![vec![1, 2, 13], vec![2, 1, 7], vec![0, 1, 9]];
        let radius = 2;
        let ans = vec![1, 2];
        assert_eq!(best_coordinate(towers, radius), ans);
    }
}
