/*
 * @Date: 2021-06-06 09:36:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-06 09:44:49
 */

#include <cassert>
#include <string>
#include <vector>
using namespace std;

int findMaxForm(vector<string>& strs, int m, int n) {
  vector<vector<int>> dp(m + 1, vector<int>(n + 1, 0));
  for (string str : strs) {
    int zeroNum = 0, oneNum = 0;
    for (char c : str) {  // 统计每个字符串的0和1的数量
      if (c == '0') {
        zeroNum++;
      } else {
        oneNum++;
      }
    }
    for (int i = m; i >= zeroNum; i--) {  // 01背包罢了, 但是重量有两个维度
      for (int j = n; j >= oneNum; j--) {
        dp[i][j] = max(dp[i][j], dp[i - zeroNum][j - oneNum] + 1);
      }  // 不装 or 装得下, 取最大值
    }
  }
  return dp[m][n];
}

int main() {
  {
    vector<string> strs{"10", "0001", "111001", "1", "0"};
    int m = 5, n = 3;
    assert(findMaxForm(strs, m, n) == 4);
  }
  {
    vector<string> strs{"01", "0", "1"};
    int m = 1, n = 1;
    assert(findMaxForm(strs, m, n) == 2);
  }
}