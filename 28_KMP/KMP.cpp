/*
 * @Date: 2021-04-20 08:57:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-20 09:22:52
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

int KMP(string haystack, string needle) {
  int n = haystack.size(), m = needle.size();
  if (m == 0) return 0;
  vector<int> pi(m);
  for (int i = 1, j = 0; i < m; i++) {
    while (j > 0 && needle[i] != needle[j]) j = pi[j - 1];
    if (needle[i] == needle[j]) j++;
    pi[i] = j;
  }

  for (int i = 0, j = 0; i < n; i++) {
    while (j > 0 && haystack[i] != needle[j]) j = pi[j - 1];
    if (haystack[i] == needle[j]) j++;
    if (j == m) return i - m + 1;
  }
  return -1;
}

int main() {
  {
    string haystack = "hello";
    string needle = "ll";
    assert(KMP(haystack, needle) == 2);
  }
  {
    string haystack = "aaaaa";
    string needle = "bba";
    assert(KMP(haystack, needle) == -1);
  }
  {
    string haystack = "";
    string needle = "";
    assert(KMP(haystack, needle) == 0);
  }
  return 0;
}
