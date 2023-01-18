/*
 * @Date: 2022-01-22 09:13:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-22 09:18:33
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int removePalindromeSub(string s) {
    int n = s.size();
    for (int i = 0; i < n / 2; ++i) {
      if (s[i] != s[n - 1 - i]) {
        return 2;
      }
    }
    return 1;
  }
};

int main() {
  assert(Solution().removePalindromeSub("ababa") == 1);
  assert(Solution().removePalindromeSub("abb") == 2);
  assert(Solution().removePalindromeSub("babb") == 2);
}