/*
 * @Date: 2021-12-11 07:08:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-11 07:27:34
 */

struct TopVotedCandidate {
    top_voted_candidate: Vec<i32>,
    times: Vec<i32>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let (len, mut top_votes, mut top_person) = (times.len(), -1, -1);
        let (mut persons_stats, mut top_voted_candidate) = (vec![0; len], vec![0; len]);
        times.iter().enumerate().for_each(|(i, _)| {
            persons_stats[persons[i] as usize] += 1;
            if persons_stats[persons[i] as usize] >= top_votes {
                top_votes = persons_stats[persons[i] as usize];
                top_person = persons[i];
            }
            top_voted_candidate[i] = top_person;
        });
        TopVotedCandidate {
            top_voted_candidate,
            times,
        }
    }

    fn q(&self, t: i32) -> i32 {
        let mut t_index = self.times.len() - 1;
        let (mut left, mut right) = (0, t_index);
        if t < self.times[t_index] {
            loop {
                let m = (right + left) / 2;
                if t == self.times[m] {
                    t_index = m as usize;
                    break;
                } else if t < self.times[m] {
                    right = m as usize;
                } else {
                    left = m as usize;
                }
                if left + 1 == right {
                    t_index = left;
                    break;
                }
            }
        }
        self.top_voted_candidate[t_index]
    }
}

fn main() {
    let obj = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
    assert_eq!(obj.q(3), 0);
    assert_eq!(obj.q(12), 1);
    assert_eq!(obj.q(25), 1);
    assert_eq!(obj.q(15), 0);
    assert_eq!(obj.q(24), 0);
    assert_eq!(obj.q(8), 1);
}
