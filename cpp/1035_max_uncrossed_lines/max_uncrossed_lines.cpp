/*
 * @Date: 2021-05-21 08:42:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-21 08:57:12
 */

#include <cassert>
#include <vector>
using namespace std;

int maxUncrossedLines(const vector<int>& nums1, const vector<int>& nums2) {
  int m = nums1.size(), n = nums2.size();
  vector<vector<int>> dp(m + 1, vector<int>(n + 1));
  for (int i = 1; i <= m; i++) {
    int num1 = nums1[i - 1];
    for (int j = 1; j <= n; j++) {
      int num2 = nums2[j - 1];
      if (num1 == num2) {
        dp[i][j] = dp[i - 1][j - 1] + 1;
      } else {
        dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
      }
    }
  }
  return dp[m][n];
}

int main() {
  assert(maxUncrossedLines(vector<int>{1, 4, 2}, vector<int>{1, 2, 4}) == 2);
  assert(maxUncrossedLines(vector<int>{2, 5, 1, 2, 5},
                           vector<int>{10, 5, 2, 1, 5, 2}) == 3);
  assert(maxUncrossedLines(vector<int>{1, 3, 7, 1, 7, 5},
                           vector<int>{1, 9, 2, 5, 1}) == 2);
  return 0;
}