/*
 * @Date: 2024-01-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-13
 * @FilePath: /algorithm/cpp/2182_repeat_limited_string/repeat_limited_string.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string repeatLimitedString(string s, int repeatLimit) {
    const int N = 26;
    vector<int> count(N);
    for (char c : s) {
      count[c - 'a']++;
    }
    string ret;
    int m = 0;
    for (int i = N - 1, j = N - 2; i >= 0 && j >= 0;) {
      if (count[i] == 0) {  // 当前字符已经填完，填入后面的字符，重置 m
        m = 0;
        i--;
      } else if (m < repeatLimit) {  // 当前字符未超过限制
        count[i]--;
        ret.push_back('a' + i);
        m++;
      } else if (j >= i || count[j] == 0) {  // 当前字符已经超过限制，查找可填入的其他字符
        j--;
      } else {  // 当前字符已经超过限制，填入其他字符，并且重置 m
        count[j]--;
        ret.push_back('a' + j);
        m = 0;
      }
    }
    return ret;
  }
};

int main() {
  vector<tuple<string, int, string>> tests{
      {"cczazcc", 3, "zzcccac"},
      {"aababab", 2, "bbabaa"},
  };

  for (auto &[s, repeatLimit, expected] : tests) {
    assert(Solution().repeatLimitedString(s, repeatLimit) == expected);
  }
}