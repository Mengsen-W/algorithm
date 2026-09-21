#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<long long> resultArray(vector<int>& nums, int k) {
    int n = nums.size();
    vector<long long> result(k);
    vector<long long> dp(k);  // 初始状态，表示尚未处理任何元素，因此不存在非空子数组

    for (int i = 0; i < n; i++) {
      vector<long long> ndp(k);  // 当前层状态（滚动数组）

      ndp[nums[i] % k]++;

      for (int r = 0; r < k; r++) {
        ndp[(long long)r * nums[i] % k] += dp[r];
      }

      dp = std::move(ndp);  // 更新状态

      // 累加答案
      for (int r = 0; r < k; r++) {
        result[r] += dp[r];
      }
    }

    return result;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<long long>>> tests{
      {{1, 2, 3, 4, 5}, 3, {9, 2, 4}},
      {{1, 2, 4, 8, 16, 32}, 4, {18, 1, 2, 0}},
      {{1, 1, 2, 1, 1}, 2, {9, 6}},
  };

  for (auto& [nums, k, expected] : tests) {
    assert(Solution().resultArray(nums, k) == expected);
  }
}
