struct Solution;

use std::collections::HashMap;

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        for i in 0..size {
            parent[i] = i;
        }
        Self { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn join(&mut self, u: usize, v: usize) {
        let root_u = self.find(u);
        let root_v = self.find(v);
        if root_u != root_v {
            self.parent[root_v] = root_u;
        }
    }
}

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;
        let mut dsu = DSU::new(c + 1);

        for p in connections {
            dsu.join(p[0] as usize, p[1] as usize);
        }

        let mut online = vec![true; c + 1];
        let mut offline_counts = vec![0; c + 1];
        let mut minimum_online_stations: HashMap<usize, i32> = HashMap::new();

        for q in &queries {
            let op = q[0];
            let x = q[1] as usize;
            if op == 2 {
                online[x] = false;
                offline_counts[x] += 1;
            }
        }

        for i in 1..=c {
            let root = dsu.find(i);
            if !minimum_online_stations.contains_key(&root) {
                minimum_online_stations.insert(root, -1);
            }

            let station = *minimum_online_stations.get(&root).unwrap();
            if online[i] {
                if station == -1 || station > i as i32 {
                    minimum_online_stations.insert(root, i as i32);
                }
            }
        }

        let mut ans = Vec::new();

        for i in (0..queries.len()).rev() {
            let op = queries[i][0];
            let x = queries[i][1] as usize;
            let root = dsu.find(x);
            let station = *minimum_online_stations.get(&root).unwrap();

            if op == 1 {
                if online[x] {
                    ans.push(x as i32);
                } else {
                    ans.push(station);
                }
            }

            if op == 2 {
                if offline_counts[x] > 1 {
                    offline_counts[x] -= 1;
                } else {
                    online[x] = true;
                    if station == -1 || station > x as i32 {
                        minimum_online_stations.insert(root, x as i32);
                    }
                }
            }
        }

        ans.reverse();
        ans
    }
}

fn main() {
    let tests = vec![
        (
            5,
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
            vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]],
            vec![3, 2, 3],
        ),
        (
            3,
            vec![],
            vec![vec![1, 1], vec![2, 1], vec![1, 1]],
            vec![1, -1],
        ),
    ];

    for (c, collections, queries, expected) in tests {
        assert_eq!(Solution::process_queries(c, collections, queries), expected);
    }
}
