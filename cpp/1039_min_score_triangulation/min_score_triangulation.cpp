
#include <cassert>
#include <functional>
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
  {
    vector<int> values{1, 2, 3};
    int ans = 6;
    assert(Solution().minScoreTriangulation(values) == ans);
  }

  {
    vector<int> values{3, 7, 4, 5};
    int ans = 144;
    assert(Solution().minScoreTriangulation(values) == ans);
  }

  {
    vector<int> values{1, 3, 1, 4, 1, 5};
    int ans = 13;
    assert(Solution().minScoreTriangulation(values) == ans);
  }
}
