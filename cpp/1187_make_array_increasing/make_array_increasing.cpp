/*
 * @Date: 2023-04-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-20
 * @FilePath: /algorithm/cpp/1187_make_array_increasing/make_array_increasing.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

constexpr int INF = 0x3f3f3f3f;

class Solution {
 public:
  int makeArrayIncreasing(vector<int>& arr1, vector<int>& arr2) {
    sort(arr2.begin(), arr2.end());
    arr2.erase(unique(arr2.begin(), arr2.end()), arr2.end());
    int n = arr1.size();
    int m = arr2.size();
    vector<vector<int>> dp(n + 1, vector<int>(min(m, n) + 1, INF));
    dp[0][0] = -1;
    for (int i = 1; i <= n; i++) {
      for (int j = 0; j <= min(i, m); j++) {
        /* 如果当前元素大于序列的最后一个元素 */
        if (arr1[i - 1] > dp[i - 1][j]) {
          dp[i][j] = arr1[i - 1];
        }
        if (j > 0 && dp[i - 1][j - 1] != INF) {
          /* 查找严格大于 dp[i - 1][j - 1] 的最小元素 */
          auto it = upper_bound(arr2.begin() + j - 1, arr2.end(), dp[i - 1][j - 1]);
          if (it != arr2.end()) {
            dp[i][j] = min(dp[i][j], *it);
          }
        }
        if (i == n && dp[i][j] != INF) {
          return j;
        }
      }
    }
    return -1;
  }
};

int main() {
  {
    vector<int> arr1{1, 5, 3, 5, 7};
    vector<int> arr2{1, 3, 2, 4};
    int ans = 1;
    assert(Solution().makeArrayIncreasing(arr1, arr2) == ans);
  }

  {
    vector<int> arr1{1, 5, 3, 6, 7};
    vector<int> arr2{4, 3, 1};
    int ans = 2;
    assert(Solution().makeArrayIncreasing(arr1, arr2) == ans);
  }

  {
    vector<int> arr1{1, 5, 3, 5, 7};
    vector<int> arr2{1, 6, 3, 3};
    int ans = -1;
    assert(Solution().makeArrayIncreasing(arr1, arr2) == ans);
  }
}