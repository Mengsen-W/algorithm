#include <cassert>
#include <climits>
#include <cstdlib>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int maximumJumps(vector<int>& nums, int target) {
    int n = nums.size();
    vector<int> dp(n, INT_MIN);
    dp[0] = 0;
    for (int i = 1; i < n; i++) {
      for (int j = 0; j < i; j++) {
        if (abs(nums[j] - nums[i]) <= target) {
          dp[i] = max(dp[i], dp[j] + 1);
        }
      }
    }

    return dp[n - 1] < 0 ? -1 : dp[n - 1];
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 3, 6, 4, 1, 2}, 2, 3},
      {{1, 3, 6, 4, 1, 2}, 3, 5},
      {{1, 3, 6, 4, 1, 2}, 0, -1},
  };

  for (auto [nums, target, expected] : tests) {
    assert(Solution().maximumJumps(nums, target) == expected);
  }
}