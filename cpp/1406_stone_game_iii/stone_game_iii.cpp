#include <cassert>
#include <climits>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  string stoneGameIII(vector<int>& stoneValue) {
    int n = stoneValue.size();
    vector<int> dp(n + 1, INT_MIN);
    // 边界情况，当没有石子时，分数为 0，每个位置存储的是 alice 比 bob 多的部分
    dp[n] = 0;

    for (int i = n - 1; i >= 0; --i) {
      int current_sum = 0;  // 当前选择拿的总量
      for (int j = i + 1; j <= i + 3 && j <= n; ++j) {
        current_sum += stoneValue[j - 1];
        dp[i] = max(dp[i], current_sum - dp[j]);
      }
    }
    if (dp[0] == 0) {
      return "Tie";
    }
    if (dp[0] > 0) {
      return "Alice";
    }
    return "Bob";
  }
};

int main() {
  vector<tuple<vector<int>, string>> tests{
      {{1, 2, 3, 7}, "Bob"},
      {{1, 2, 3, -9}, "Alice"},
      {{1, 2, 3, 6}, "Tie"},
  };

  for (auto [nums, ans] : tests) {
    assert(Solution().stoneGameIII(nums) == ans);
  }
  return 0;
}