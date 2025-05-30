struct Solution;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let get = |x| {
            let mut d = vec![-1; n];
            let mut p = x;
            let mut dis = 0;
            while p != -1 && d[p as usize] == -1 {
                d[p as usize] = dis;
                p = edges[p as usize];
                dis += 1;
            }
            d
        };
        let d1 = get(node1);
        let d2 = get(node2);
        let mut res = -1;
        for i in 0..n {
            if d1[i] != -1
                && d2[i] != -1
                && (res == -1 || d1[res as usize].max(d2[res as usize]) > d1[i].max(d2[i]))
            {
                res = i as i32;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![2, 2, 3, -1], 0, 1, 2), (vec![1, 2, -1], 0, 2, 2)];

    for (edges, node1, node2, expected) in tests {
        assert_eq!(
            Solution::closest_meeting_node(edges, node1, node2),
            expected
        );
    }
}
