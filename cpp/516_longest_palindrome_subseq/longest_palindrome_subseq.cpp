/*
 * @Date: 2021-08-12 14:18:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-12 15:15:13
 */

#include <cassert>
#include <functional>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestPalindromeSubseq(string s) {
    int n = s.length();
    vector<vector<int>> dp(n, vector<int>(n));
    for (int i = n - 1; i >= 0; i--) {
      dp[i][i] = 1;
      char c1 = s[i];
      for (int j = i + 1; j < n; j++) {
        char c2 = s[j];
        if (c1 == c2)
          dp[i][j] = dp[i + 1][j - 1] + 2;
        else
          dp[i][j] = max(dp[i + 1][j], dp[i][j - 1]);
      }
    }
    return dp[0][n - 1];
  }
};

int main() {
  {
    string s = "bbbab";
    assert(Solution{}.longestPalindromeSubseq(s) == 4);
  }
  {
    string s = "cbbd";
    assert(Solution{}.longestPalindromeSubseq(s) == 2);
  }
}