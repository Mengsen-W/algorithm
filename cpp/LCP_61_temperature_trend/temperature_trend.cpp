#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int temperatureTrend(vector<int>& temperatureA, vector<int>& temperatureB) {
    auto getTrend = [](int x, int y) -> int {
      if (x == y) {
        return 0;
      }
      return x < y ? -1 : 1;
    };

    int n = temperatureA.size();
    int ans = 0, cur = 0;
    for (int i = 1; i < n; ++i) {
      int ta = getTrend(temperatureA[i - 1], temperatureA[i]);
      int tb = getTrend(temperatureB[i - 1], temperatureB[i]);
      if (ta == tb) {
        ++cur;
        ans = max(ans, cur);
      } else {
        cur = 0;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{21, 18, 18, 18, 31}, {34, 32, 16, 16, 17}, 2},
      {{5, 10, 16, -6, 15, 11, 3}, {16, 22, 23, 23, 25, 3, -16}, 3},
  };

  for (auto& [temperatureA, temperatureB, ans] : tests) {
    assert(Solution().temperatureTrend(temperatureA, temperatureB) == ans);
  }
}