struct Solution;

use std::collections::HashMap;
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if self.rank[x_root] >= self.rank[y_root] {
            self.parent[y_root] = x_root;
        } else {
            self.parent[x_root] = y_root;
        }
        if self.rank[x_root] == self.rank[y_root] {
            self.rank[x_root] += 1;
        }
    }
}
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let n = accounts.len();
        let mut uf: UnionFind = UnionFind::new(n);
        let mut res: Vec<Vec<String>> = Vec::new();
        let mut email_to_id: HashMap<String, usize> = HashMap::new();
        for i in 0..n {
            for email in accounts[i].iter().skip(1) {
                let t = email_to_id.entry(email.clone()).or_insert(i); //bind email to account_id
                uf.union(i, *t); //the accounts[i] point to the same parent (account_id)
            }
        }

        let mut id_to_email: HashMap<usize, Vec<String>> = HashMap::new();
        for (k, v) in email_to_id.iter_mut() {
            let account_id = uf.find(*v); //find the parent of this email
            let t = id_to_email.entry(account_id).or_insert(Vec::new());
            t.push(k.clone()); // add email for this parent
        }

        for (k, v) in id_to_email.iter_mut() {
            v.sort_unstable(); // sort by ascii order
            v.insert(0, accounts[*k][0].clone());
            res.push(v.to_vec());
        }

        res
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec!["John", "johnsmith@mail.com", "john00@mail.com"],
                vec!["John", "johnnybravo@mail.com"],
                vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
                vec!["Mary", "mary@mail.com"],
            ],
            vec![
                vec![
                    "John",
                    "john00@mail.com",
                    "john_newyork@mail.com",
                    "johnsmith@mail.com",
                ],
                vec!["John", "johnnybravo@mail.com"],
                vec!["Mary", "mary@mail.com"],
            ],
        ),
        (
            vec![
                vec!["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
                vec!["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
                vec!["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
                vec!["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
                vec!["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"],
            ],
            vec![
                vec!["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
                vec!["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
                vec!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
                vec!["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
                vec!["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"],
            ],
        ),
    ];

    for (account, ans) in tests {
        assert_eq!(
            Solution::accounts_merge(
                account
                    .into_iter()
                    .map(|v| v.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            ),
            ans.into_iter()
                .map(|v| v.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );
    }
}
