#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  bool predictTheWinner(vector<int>& nums) {
    int n = nums.size();
    vector<vector<int>> dp(n, vector<int>(n));  // dp[i][j] 表示在[i, j]区间内先手能获得的最大分数
    for (int i = 0; i < n; ++i) {
      dp[i][i] = nums[i];  // 只有一个数，先手只能拿这个数
    }

    for (int size = 2; size <= n; ++size) {  // 从2个区间开始算
      for (int i = 0; i <= n - size; ++i) {  // 计算 i 到 j 的区间
        int j = i + size - 1;                // j 的区间 通过 j <= n - 1 反推 i <= n - size
        dp[i][j] = max(nums[i] - dp[i + 1][j],
                       nums[j] - dp[i][j - 1]);  // 如果下一手左边就是 i+1 下一手右边就是 j-1 取对比下一手能赢的差值
      }
    }
    return dp[0][n - 1] >= 0;
  }
  // 可以优化空间复杂度
  bool predictTheWinner2(vector<int>& nums) {
    int n = nums.size();
    vector<int> dp(n);
    // 从下往上算
    for (int i = n - 1; i >= 0; --i) {
      dp[i] = nums[i];
      for (int j = i + 1; j < n; ++j) {
        // 没更新之前 dp[j] 表示 [i + 1, j] 区间内先手能获得的最大分数
        dp[j] = max(nums[i] - dp[j], nums[j] - dp[j - 1]);
      }
    }
    return dp[n - 1] >= 0;
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{1, 5, 2}, false},
      {{1, 5, 233, 7}, true},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().predictTheWinner(nums) == ans);
    assert(Solution().predictTheWinner2(nums) == ans);
  }
}
