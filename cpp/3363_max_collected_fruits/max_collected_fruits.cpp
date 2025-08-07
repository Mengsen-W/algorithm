#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxCollectedFruits(vector<vector<int>>& fruits) {
    int n = fruits.size();
    int ans = 0;
    for (int i = 0; i < n; ++i) {
      ans += fruits[i][i];
    }

    auto dp = [&]() -> int {
      vector<int> prev(n, INT_MIN), curr(n, INT_MIN);
      prev[n - 1] = fruits[0][n - 1];
      for (int i = 1; i < n - 1; ++i) {
        for (int j = max(n - 1 - i, i + 1); j < n; ++j) {
          int best = prev[j];
          if (j - 1 >= 0) {
            best = max(best, prev[j - 1]);
          }
          if (j + 1 < n) {
            best = max(best, prev[j + 1]);
          }
          curr[j] = best + fruits[i][j];
        }
        swap(prev, curr);
      }
      return prev[n - 1];
    };

    ans += dp();

    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < i; ++j) {
        swap(fruits[j][i], fruits[i][j]);
      }
    }

    ans += dp();
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 2, 3, 4}, {5, 6, 8, 7}, {9, 10, 11, 12}, {13, 14, 15, 16}}, 100},
      {{{1, 1}, {1, 1}}, 4},
  };

  for (auto& [fruits, ans] : tests) {
    assert(Solution().maxCollectedFruits(fruits) == ans);
  }
}