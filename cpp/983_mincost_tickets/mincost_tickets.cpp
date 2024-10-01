#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 private:
  vector<int> days, costs;
  vector<int> memo;
  int durations[3] = {1, 7, 30};

 public:
  int mincostTickets(vector<int>& days, vector<int>& costs) {
    this->days = days;
    this->costs = costs;
    memo.assign(days.size(), -1);
    return dp(0);
  }

  int dp(int i) {
    if (i >= days.size()) {
      return 0;
    }
    if (memo[i] != -1) {
      return memo[i];
    }
    memo[i] = INT_MAX;
    int j = i;
    for (int k = 0; k < 3; ++k) {
      while (j < days.size() && days[j] < days[i] + durations[k]) {
        ++j;
      }
      memo[i] = min(memo[i], dp(j) + costs[k]);
    }
    return memo[i];
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{1, 4, 6, 7, 8, 20}, {2, 7, 15}, 11},
      {{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31}, {2, 7, 15}, 17},
  };

  for (auto &[days, costs, ans] : tests) {
    assert(Solution().mincostTickets(days, costs) == ans);
  }
}