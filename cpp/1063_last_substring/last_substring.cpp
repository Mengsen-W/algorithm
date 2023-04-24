/*
 * @Date: 2023-04-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-24
 * @FilePath: /algorithm/cpp/1063_last_substring/last_substring.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string lastSubstring(string s) {
    int i = 0, j = 1, n = s.size();
    while (j < n) {
      int k = 0;
      while (j + k < n && s[i + k] == s[j + k]) {
        k++;
      }
      if (j + k < n && s[i + k] < s[j + k]) {
        int t = i;
        i = j;
        j = max(j + 1, t + k + 1);
      } else {
        j = j + k + 1;
      }
    }
    return s.substr(i, n - i);
  }
};

int main() {
  {
    string s{"abab"};
    string ans{"bab"};
    assert(Solution().lastSubstring(s) == ans);
  }

  {
    string s{"leetcode"};
    string ans{"tcode"};
    assert(Solution().lastSubstring(s) == ans);
  }
}