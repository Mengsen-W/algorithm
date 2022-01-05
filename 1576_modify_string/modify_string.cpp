/*
 * @Date: 2022-01-05 01:15:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-05 01:34:19
 */

#include <cassert>
#include <iostream>
#include <string>

using namespace std;

class Solution {
 public:
  string modifyString(string s) {
    int n = s.size();
    for (int i = 0; i < n; ++i) {
      if (s[i] == '?') {
        for (char ch = 'a'; ch <= 'c'; ++ch) {
          if ((i > 0 && s[i - 1] == ch) || (i < n - 1 && s[i + 1] == ch)) {
            continue;
          }
          s[i] = ch;
          break;
        }
      }
    }
    return s;
  }
};

int main() {
  assert(Solution().modifyString("?zs") == "azs");
  assert(Solution().modifyString("ubv?w") == "ubvaw");
  assert(Solution().modifyString("j?qg??b") == "jaqgacb");
  assert(Solution().modifyString("??yw?ipkj?") == "abywaipkja");
}
