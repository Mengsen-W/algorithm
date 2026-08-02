#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  bool stoneGame(vector<int>& piles) {
    int length = piles.size();
    auto dp = vector<int>(length);
    for (int i = length - 1; i >= 0; i--) {
      dp[i] = piles[i];
      for (int j = i + 1; j < length; j++) {
        dp[j] = max(piles[i] - dp[j], piles[j] - dp[j - 1]);
      }
    }
    return dp[length - 1] > 0;
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{5, 3, 4, 5}, true},
      {{3, 7, 2, 3}, true},
  };

  for (auto& [piles, ans] : tests) {
    assert(Solution().stoneGame(piles) == ans);
  }
}
