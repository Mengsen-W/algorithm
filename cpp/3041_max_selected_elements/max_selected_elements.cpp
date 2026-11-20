#include <algorithm>
#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
 public:
  int maxSelectedElements(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    unordered_map<int, int> dp;
    int ans = 0;
    for (int x : nums) {
      dp[x + 1] = max(dp[x + 1], dp[x] + 1);
      dp[x] = max(dp[x], dp[x - 1] + 1);
      ans = max(ans, max(dp[x], dp[x + 1]));
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 1, 5, 1, 1}, 3},
      {{1, 4, 7, 10}, 1},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().maxSelectedElements(nums) == expected);
  }
}