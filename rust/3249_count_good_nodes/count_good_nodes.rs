struct Solution;

impl Solution {
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        let mut res = 0;
        for edge in edges {
            let (x, y) = (edge[0] as usize, edge[1] as usize);
            g[x].push(y);
            g[y].push(x);
        }

        fn dfs(node: usize, parent: isize, g: &Vec<Vec<usize>>, res: &mut i32) -> usize {
            let mut valid = true;
            let mut tree_size = 0;
            let mut sub_tree_size = 0;

            for &child in &g[node] {
                if child != parent as usize {
                    let size = dfs(child, node as isize, g, res);
                    if sub_tree_size == 0 {
                        sub_tree_size = size;
                    } else if size != sub_tree_size {
                        valid = false;
                    }
                    tree_size += size;
                }
            }
            if valid {
                *res += 1;
            }
            tree_size + 1
        }

        dfs(0, -1, &g, &mut res);
        res
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
            ],
            7,
        ),
        (
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![0, 5],
                vec![1, 6],
                vec![2, 7],
                vec![3, 8],
            ],
            6,
        ),
        (
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![0, 5],
                vec![5, 6],
                vec![6, 7],
                vec![7, 8],
                vec![0, 9],
                vec![9, 10],
                vec![9, 12],
                vec![10, 11],
            ],
            12,
        ),
    ];

    for (edges, ans) in tests {
        assert_eq!(Solution::count_good_nodes(edges) == ans);
    }
}
