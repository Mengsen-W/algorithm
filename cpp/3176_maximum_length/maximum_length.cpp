#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumLength(vector<int>& nums, int k) {
    int ans = 0;
    int len = nums.size();
    vector<vector<int>> dp;
    dp.resize(len, vector<int>(51, -1));

    for (int i = 0; i < len; i++) {
      dp[i][0] = 1;
      for (int l = 0; l <= k; l++) {
        for (int j = 0; j < i; j++) {
          int add = (nums[i] != nums[j]);
          if (l - add >= 0 && dp[j][l - add] != -1) {
            dp[i][l] = max(dp[i][l], dp[j][l - add] + 1);
          }
        }
        ans = max(ans, dp[i][l]);
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 1, 1, 3}, 2, 4},
      {{1, 2, 3, 4, 5, 1}, 0, 2},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().maximumLength(nums, k) == ans);
  }
}