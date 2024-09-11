#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximizeWin(vector<int>& prizePositions, int k) {
    int n = prizePositions.size();
    vector<int> dp(n + 1);
    int ans = 0;
    for (int left = 0, right = 0; right < n; right++) {
      while (prizePositions[right] - prizePositions[left] > k) {
        left++;
      }
      ans = max(ans, right - left + 1 + dp[left]);
      dp[right + 1] = max(dp[right], right - left + 1);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 1, 2, 2, 3, 3, 5}, 2, 7},
      {{1, 2, 3, 4}, 0, 2},
  };

  for (auto &[prizePositions, k, ans] : tests) {
    assert(Solution().maximizeWin(prizePositions, k) == ans);
  }
}