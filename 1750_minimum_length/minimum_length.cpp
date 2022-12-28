/*
 * @Date: 2022-12-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-28
 * @FilePath: /algorithm/1750_minimum_length/minimum_length.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int minimumLength(string s) {
    int n = s.size();
    int left = 0, right = n - 1;
    while (left < right && s[left] == s[right]) {
      char c = s[left];
      while (left <= right && s[left] == c) {
        left++;
      }
      while (left <= right && s[right] == c) {
        right--;
      }
    }
    return right - left + 1;
  }
};

int main() {
  {
    string s{"ca"};
    int ans = 2;
    assert(Solution().minimumLength(s) == ans);
  }

  {
    string s{"cabaabac"};
    int ans = 0;
    assert(Solution().minimumLength(s) == ans);
  }

  {
    string s{"aabccabba"};
    int ans = 3;
    assert(Solution().minimumLength(s) == ans);
  }
}
