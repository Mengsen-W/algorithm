/*
 * @Date: 2022-12-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-26
 * @FilePath: /algorithm/1759_count_homogenous/count_homogenous.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int countHomogenous(string s) {
    long long res = 0;
    long long mod = 1e9 + 7;
    int prev = s[0];
    int cnt = 0;
    for (auto c : s) {
      if (c == prev) {
        cnt++;
      } else {
        res += (long long)(cnt + 1) * cnt / 2;
        cnt = 1;
        prev = c;
      }
    }
    res += (long long)(cnt + 1) * cnt / 2;
    return res % mod;
  }
};

int main() {
  {
    string s{"abbcccaa"};
    int ans = 13;
    assert(Solution().countHomogenous(s) == ans);
  }

  {
    string s{"xy"};
    int ans = 2;
    assert(Solution().countHomogenous(s) == ans);
  }

  {
    string s{"zzzzz"};
    int ans = 15;
    assert(Solution().countHomogenous(s) == ans);
  }
}