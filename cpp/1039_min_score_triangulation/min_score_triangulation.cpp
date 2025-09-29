
#include <cassert>
#include <climits>
#include <functional>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int minScoreTriangulation(vector<int>& values) {
    unordered_map<int, int> memo;
    int n = values.size();
    function<int(int, int)> dp = [&](int i, int j) -> int {
      if (i + 2 > j) {
        return 0;
      }
      if (i + 2 == j) {
        return values[i] * values[i + 1] * values[j];
      }
      int key = i * n + j;
      if (!memo.count(key)) {
        int minScore = INT_MAX;
        for (int k = i + 1; k < j; k++) {
          minScore = min(minScore, values[i] * values[k] * values[j] + dp(i, k) + dp(k, j));
        }
        memo[key] = minScore;
      }
      return memo[key];
    };
    return dp(0, n - 1);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3}, 6},
      {{3, 7, 4, 5}, 144},
      {{1, 3, 1, 4, 1, 5}, 13},
  };

  for (auto& [values, expect] : tests) {
    assert(Solution().minScoreTriangulation(values) == expect);
  }
  return 0;
}
