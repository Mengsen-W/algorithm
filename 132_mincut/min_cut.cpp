/*
 * @Date: 2021-03-08 09:36:18
 * @Author: mengsen
 * @LastEditors: mengsen
 * @LastEditTime: 2021-03-08 09:44:36
 */

#include <bits/stdc++.h>

using namespace std;

int min_cut(string s) {
  int n = s.size();
  vector<vector<bool>> dp(n, vector<bool>(n));
  for (int l = 0; l < n; l++) {
    for (int i = 0; i + l < n; i++) {
      int j = i + l;
      if (i == j)
        dp[i][j] = true;
      else if (j == i + 1)
        dp[i][j] = s[i] == s[j];
      else
        dp[i][j] = dp[i + 1][j - 1] && (s[i] == s[j]);
    }
  }
  vector<int> dp2(n, 0x3f3f3f3f);
  for (int i = 0; i < n; i++) {
    if (dp[0][i]) {
      dp2[i] = 0;
      continue;
    }
    for (int j = 0; j < i; j++) {
      if (dp[j + 1][i]) {
        dp2[i] = min(dp2[i], dp2[j] + 1);
      }
    }
  }
  return dp2[n - 1];
}

int main(void) {
  assert(min_cut("aab") == 1);
  assert(min_cut("a") == 0);
  assert(min_cut("ab") == 1);
  return 0;
}