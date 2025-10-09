#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long minTime(vector<int>& skill, vector<int>& mana) {
    int n = skill.size(), m = mana.size();
    vector<long long> times(n);
    for (int j = 0; j < m; j++) {
      long long cur_time = 0;
      for (int i = 0; i < n; i++) {
        cur_time = max(cur_time, times[i]) + skill[i] * mana[j];
      }
      times[n - 1] = cur_time;
      for (int i = n - 2; i >= 0; i--) {
        times[i] = times[i + 1] - skill[i + 1] * mana[j];
      }
    }
    return times[n - 1];
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, long long>> tests{
      {{1, 5, 2, 4}, {5, 1, 4, 2}, 110},
      {{1, 1, 1}, {1, 1, 1}, 5},
      {{1, 2, 3, 4}, {1, 2}, 21},
  };

  for (auto& [skill, mana, expect] : tests) {
    assert(Solution().minTime(skill, mana) == expect);
  }
}