/*
 * @Date: 2021-03-17 08:26:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-17 08:29:41
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

int num_distinct(string s, string t) {
  int ls = s.size(), lt = t.size();
  vector<vector<long long>> dp(lt + 1, vector<long long>(ls + 1, 0));
  for (int i = 0; i <= ls; i++) dp[0][i] = 1;
  for (int i = 1; i <= lt; i++) {
    for (int j = 1; j <= ls; j++) {
      if (s[j - 1] == t[i - 1])
        dp[i][j] = dp[i - 1][j - 1] + dp[i][j - 1];
      else
        dp[i][j] = dp[i][j - 1];
    }
  }
  return dp[lt][ls];
}

int main() {
  assert(num_distinct(string("rabbbit"), string("rabbit")) == 3);
  assert(num_distinct(string("babgbag"), string("bag")) == 5);
  return 0;
}