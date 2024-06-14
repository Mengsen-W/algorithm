#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  long long maxScore(vector<int>& nums, int x) {
    long long res = nums[0];
    vector<long long> dp(2, INT_MIN);
    dp[nums[0] % 2] = nums[0];
    for (int i = 1; i < nums.size(); i++) {
      int parity = nums[i] % 2;
      long long cur = max(dp[parity] + nums[i], dp[1 - parity] - x + nums[i]);
      res = max(res, cur);
      dp[parity] = max(dp[parity], cur);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{2, 3, 6, 1, 9, 2}, 5, 13},
      {{2, 4, 6, 8}, 3, 20},
  };

  for (auto& [nums, x, ans] : tests) {
    assert(Solution().maxScore(nums, x) == ans);
  }
}