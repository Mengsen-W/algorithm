#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxScoreSightseeingPair(vector<int>& values) {
    int ans = 0, mx = values[0] + 0;
    for (int j = 1; j < values.size(); ++j) {
      ans = max(ans, mx + values[j] - j);
      // 边遍历边维护
      mx = max(mx, values[j] + j);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{8, 1, 5, 2, 6}, 11},
      {{1, 2}, 2},
  };

  for (auto &[values, ans] : tests) {
    assert(Solution().maxScoreSightseeingPair(values) == ans);
  }
}