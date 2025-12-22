#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minDeletionSize(vector<string>& strs) {
    int n = strs[0].size();
    vector<int> dp(n, 1);

    for (int i = n - 2; i >= 0; i--) {
      for (int j = i + 1; j < n; j++) {
        bool valid = true;
        for (const auto& row : strs) {
          if (row[i] > row[j]) {
            valid = false;
            break;
          }
        }
        if (valid) {
          dp[i] = max(dp[i], 1 + dp[j]);
        }
      }
    }

    return n - *max_element(dp.begin(), dp.end());
  }
};

int main() {
  vector<tuple<vector<string>, int>> tests{
      {{"babca", "bbazb"}, 3},
      {{"edcba"}, 4},
      {{"ghi", "def", "abc"}, 0},
  };

  for (auto& [strs, ans] : tests) {
    assert(Solution().minDeletionSize(strs) == ans);
  }
  return 0;
}