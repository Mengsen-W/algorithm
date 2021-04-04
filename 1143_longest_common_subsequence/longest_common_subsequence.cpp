/*
 * @Date: 2021-04-04 19:01:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-04 19:11:57
 */

#include <cassert>
#include <string>
#include <vector>
using namespace std;

int longest_common_subsequence(string text1, string text2) {
  vector<vector<int>> dp(text1.size() + 1, vector<int>(text2.size() + 1, 0));
  for (int i = 1; i <= text1.size(); i++) {
    for (int j = 1; j <= text2.size(); j++) {
      if (text1[i - 1] == text2[j - 1]) {
        dp[i][j] = dp[i - 1][j - 1] + 1;
      } else {
        dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
      }
    }
  }
  return dp[text1.size()][text2.size()];
}

int main() {
  {
    string text1 = "abcde";
    string text2 = "ace";
    assert(3 == longest_common_subsequence(text1, text2));
  }
  {
    string text1 = "abc";
    string text2 = "abc";
    assert(3 == longest_common_subsequence(text1, text2));
  }
  {
    string text1 = "abc";
    string text2 = "def";
    assert(0 == longest_common_subsequence(text1, text2));
  }
}