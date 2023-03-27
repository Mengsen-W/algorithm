/*
 * @Date: 2023-03-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-27
 * @FilePath: /algorithm/cpp/1638_count_substrings/count_substrings.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int countSubstrings(string s, string t) {
    int m = s.size(), n = t.size();
    vector<vector<int>> dpl(m + 1, vector<int>(n + 1));
    vector<vector<int>> dpr(m + 1, vector<int>(n + 1));
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        dpl[i + 1][j + 1] = s[i] == t[j] ? (dpl[i][j] + 1) : 0;
      }
    }
    for (int i = m - 1; i >= 0; i--) {
      for (int j = n - 1; j >= 0; j--) {
        dpr[i][j] = s[i] == t[j] ? (dpr[i + 1][j + 1] + 1) : 0;
      }
    }
    int ans = 0;
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        if (s[i] != t[j]) {
          ans += (dpl[i][j] + 1) * (dpr[i + 1][j + 1] + 1);
        }
      }
    }
    return ans;
  }
};

int main() {
  {
    string s{"aba"};
    string t{"baba"};
    int ans = 6;
    assert(Solution().countSubstrings(s, t) == ans);
  }

  {
    string s{"ab"};
    string t{"bb"};
    int ans = 3;
    assert(Solution().countSubstrings(s, t) == ans);
  }

  {
    string s{"a"};
    string t{"a"};
    int ans = 0;
    assert(Solution().countSubstrings(s, t) == ans);
  }

  {
    string s{"abe"};
    string t{"bbc"};
    int ans = 10;
    assert(Solution().countSubstrings(s, t) == ans);
  }
}