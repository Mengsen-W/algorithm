#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxTotalReward(vector<int>& rewardValues) {
    sort(rewardValues.begin(), rewardValues.end());
    int m = rewardValues.back();
    vector<int> dp(2 * m);
    dp[0] = 1;
    for (int x : rewardValues) {
      for (int k = 2 * x - 1; k >= x; k--) {
        if (dp[k - x]) {
          dp[k] = 1;
        }
      }
    }
    int res = 0;
    for (int i = 0; i < dp.size(); i++) {
      if (dp[i]) {
        res = i;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 1, 3, 3}, 4},
      {{1, 6, 4, 3, 2}, 11},
  };

  for (auto& [rewardValues, ans] : tests) {
    assert(Solution().maxTotalReward(rewardValues) == ans);
  }
}