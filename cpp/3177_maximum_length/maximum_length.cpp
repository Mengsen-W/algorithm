#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumLength(vector<int>& nums, int k) {
    int len = nums.size();
    unordered_map<int, vector<int>> dp;
    vector<int> zd(k + 1, 0);

    for (int i = 0; i < len; i++) {
      int v = nums[i];
      if (!dp.count(v)) {
        dp[v] = vector<int>(k + 1, 0);
      }

      auto& tmp = dp[v];
      for (int j = 0; j <= k; j++) {
        ++tmp[j];
        if (j > 0) {
          tmp[j] = max(tmp[j], zd[j - 1] + 1);
        }
      }
      for (int j = 0; j <= k; j++) {
        zd[j] = max(zd[j], tmp[j]);
      }
    }
    return zd[k];
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 1, 1, 3}, 2, 4},
      {{1, 2, 3, 4, 5, 1}, 0, 2},
  };

  for (auto &[nums, k, ans] : tests) {
    assert(Solution().maximumLength(nums, k) == ans);
  }
}