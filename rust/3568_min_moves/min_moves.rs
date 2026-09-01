struct Solution;

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let dx = [0, 1, 0, -1];
        let dy = [1, 0, -1, 0];
        let m = classroom.len();
        let n = classroom[0].len();
        let mut id = vec![vec![0; n]; m];
        let mut sx = 0usize;
        let mut sy = 0usize;
        let mut cnt = 0;
        for i in 0..m {
            for j in 0..n {
                let c = classroom[i].as_bytes()[j] as char;
                if c == 'S' {
                    sx = i;
                    sy = j;
                } else if c == 'L' {
                    id[i][j] = 1 << cnt;
                    cnt += 1;
                }
            }
        }
        let full = 1 << cnt;
        let mut best_energy = vec![vec![vec![-1; full]; n]; m];
        best_energy[sx][sy][0] = energy;
        #[derive(Clone)]
        struct Info {
            x: usize,
            y: usize,
            mask: usize,
            e: i32,
            steps: i32,
        }
        let mut q: Vec<Info> = Vec::new();
        q.push(Info {
            x: sx,
            y: sy,
            mask: 0,
            e: energy,
            steps: 0,
        });
        let mut head: usize = 0;
        while head < q.len() {
            let t = q[head].clone();
            head += 1;
            if t.mask == full - 1 {
                return t.steps;
            }
            if t.e == 0 {
                continue;
            }
            for d in 0..4 {
                let nx_i = t.x as i32 + dx[d];
                let ny_i = t.y as i32 + dy[d];
                if nx_i < 0 || nx_i >= m as i32 || ny_i < 0 || ny_i >= n as i32 {
                    continue;
                }
                let nx = nx_i as usize;
                let ny = ny_i as usize;
                let c = classroom[nx].as_bytes()[ny] as char;
                if c == 'X' {
                    continue;
                }
                let ne = if c == 'R' { energy } else { t.e - 1 };
                let nmask = t.mask | id[nx][ny] as usize;
                if ne > best_energy[nx][ny][nmask] {
                    best_energy[nx][ny][nmask] = ne;
                    q.push(Info {
                        x: nx,
                        y: ny,
                        mask: nmask,
                        e: ne,
                        steps: t.steps + 1,
                    });
                }
            }
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec!["S.", "XL"], 2, 2),
        (vec!["LS", "RL"], 4, 3),
        (vec!["L.S", "RXL"], 3, -1),
    ];

    for (classroom, energy, expected) in tests {
        assert_eq!(
            Solution::min_moves(classroom.iter().map(|s| s.to_string()).collect(), energy),
            expected
        );
    }
}
