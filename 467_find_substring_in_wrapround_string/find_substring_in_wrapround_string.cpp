/*
 * @Date: 2022-05-25 09:54:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-25 10:06:22
 * @FilePath: /algorithm/467_find_substring_in_wrapround_string/find_substring_in_wrapround_string.cpp
 */

#include <cassert>
#include <numeric>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int findSubstringInWraproundString(string p) {
    vector<int> dp(26);
    int k = 0;
    for (int i = 0; i < p.length(); ++i) {
      if (i && (p[i] - p[i - 1] + 26) % 26 == 1) {  // 字符之差为 1 或 -25
        ++k;
      } else {
        k = 1;
      }
      dp[p[i] - 'a'] = max(dp[p[i] - 'a'], k);
    }
    return accumulate(dp.begin(), dp.end(), 0);
  }
};

int main() {
  assert(Solution().findSubstringInWraproundString("a") == 1);
  assert(Solution().findSubstringInWraproundString("cac") == 2);
  assert(Solution().findSubstringInWraproundString("zab") == 6);
}