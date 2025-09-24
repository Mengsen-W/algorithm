#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumLength(vector<int>& nums, int k) {
    vector<vector<int>> dp(k, vector<int>(k, 0));
    int res = 0;
    for (int num : nums) {
      num %= k;
      for (int prev = 0; prev < k; ++prev) {
        dp[prev][num] = dp[num][prev] + 1;
        res = max(res, dp[prev][num]);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 3, 4, 5}, 2, 5},
      {{1, 4, 2, 3, 1, 4}, 3, 4},
  };

  for (auto& [nums, k, expected] : tests) {
    assert(Solution().maximumLength(nums, k) == expected);
  }
}