/*
 * @Date: 2021-12-11 07:08:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-11 07:17:58
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class TopVotedCandidate {
 public:
  vector<int> tops;
  vector<int> times;

  TopVotedCandidate(vector<int>& persons, vector<int>& times) {
    unordered_map<int, int> voteCounts;
    voteCounts[-1] = -1;
    int top = -1;
    for (auto& p : persons) {
      voteCounts[p]++;
      if (voteCounts[p] >= voteCounts[top]) {
        top = p;
      }
      tops.emplace_back(top);
    }
    this->times = times;
  }

  int q(int t) {
    int pos = upper_bound(times.begin(), times.end(), t) - times.begin() - 1;
    return tops[pos];
  }
};

int main() {
  vector<int> persons = {0, 1, 1, 0, 0, 1, 0};
  vector<int> times = {0, 5, 10, 15, 20, 25, 30};
  TopVotedCandidate* t = new TopVotedCandidate(persons, times);
  assert(t->q(3) == 0);
  assert(t->q(12) == 1);
  assert(t->q(25) == 1);
  assert(t->q(15) == 0);
  assert(t->q(24) == 0);
  assert(t->q(8) == 1);
}
