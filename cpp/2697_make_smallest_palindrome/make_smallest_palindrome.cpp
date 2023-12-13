/*
 * @Date: 2023-12-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-13
 * @FilePath: /algorithm/cpp/2697_make_smallest_palindrome/make_smallest_palindrome.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string makeSmallestPalindrome(string s) {
    int left = 0, right = s.size() - 1;
    while (left < right) {
      if (s[left] != s[right]) {
        s[left] = s[right] = min(s[left], s[right]);
      }
      ++left;
      --right;
    }
    return s;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"egcfe", "efcfe"},
      {"abcd", "abba"},
      {"seven", "neven"},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().makeSmallestPalindrome(s) == ans);
  }
}