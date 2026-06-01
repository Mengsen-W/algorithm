#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumCost(vector<int>& cost) {
    sort(cost.begin(), cost.end(), greater<int>());
    int res = 0;
    int n = cost.size();
    for (int i = 0; i < n; ++i) {
      if (i % 3 != 2) {
        res += cost[i];
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3}, 5},
      {{6, 5, 7, 9, 2, 2}, 23},
      {{5, 5}, 10},
  };

  for (auto& [cost, expected] : tests) {
    assert(Solution().minimumCost(cost) == expected);
  }
  return 0;
}