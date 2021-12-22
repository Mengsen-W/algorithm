/*
 * @Date: 2021-12-22 00:44:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-22 02:04:21
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int strStr(string &haystack, string &needle) {
    int n = haystack.size(), m = needle.size();
    if (m == 0) {
      return 0;
    }
    vector<int> pi(m);
    for (int i = 1, j = 0; i < m; i++) {
      while (j > 0 && needle[i] != needle[j]) {
        j = pi[j - 1];
      }
      if (needle[i] == needle[j]) {
        j++;
      }
      pi[i] = j;
    }
    for (int i = 0, j = 0; i - j < n; i++) {
      // b 开始匹配的位置是否超过第一个叠加的 a
      while (j > 0 && haystack[i % n] != needle[j]) {
        // haystack 是循环叠加的字符串，所以取 i % n
        j = pi[j - 1];
      }
      if (haystack[i % n] == needle[j]) {
        j++;
      }
      if (j == m) {
        return i - m + 1;
      }
    }
    return -1;
  }

  int repeatedStringMatch(string a, string b) {
    int an = a.size(), bn = b.size();
    int index = strStr(a, b);
    if (index == -1) {
      return -1;
    }
    if (an - index >= bn) {
      return 1;
    }
    return (bn + index - an - 1) / an + 2;
  }
};

int main() {
  assert(Solution().repeatedStringMatch("abcd", "cdabcdab") == 3);
  assert(Solution().repeatedStringMatch("a", "aa") == 2);
  assert(Solution().repeatedStringMatch("a", "a") == 1);
  assert(Solution().repeatedStringMatch("abc", "wxyz") == -1);
}