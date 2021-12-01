/*
 * @Date: 2021-12-01 00:26:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-01 00:33:58
 */

#include <string>
#include <cassert>
using namespace std;

class Solution {
 public:
  int maxPower(string s) {
    int ans = 1, cnt = 1;
    int s_length = s.length();
    for (int i = 1; i < s_length; ++i) {
      if (s[i] == s[i - 1]) {
        ++cnt;
        ans = max(ans, cnt);
      } else {
        cnt = 1;
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().maxPower("leetcode") == 2);
  assert(Solution().maxPower("abbcccddddeeeeedcba") == 5);
  assert(Solution().maxPower("triplepillooooow") == 5);
  assert(Solution().maxPower("hooraaaaaaaaaaay") == 11);
  assert(Solution().maxPower("tourist") == 1);
}