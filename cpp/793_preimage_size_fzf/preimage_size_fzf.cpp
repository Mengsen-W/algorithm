/*
 * @Date: 2022-08-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-28
 * @FilePath: /algorithm/793_preimage_size_fzf/preimage_size_fzf.cpp
 */

#include <cassert>

using namespace std;

class Solution {
 public:
  int zeta(long x) {
    int res = 0;
    while (x) {
      res += x / 5;
      x /= 5;
    }
    return res;
  }

  long long help(int k) {
    long long r = 5LL * k;
    long long l = 0;
    while (l <= r) {
      long long mid = (l + r) / 2;
      if (zeta(mid) < k) {
        l = mid + 1;
      } else {
        r = mid - 1;
      }
    }
    return r + 1;
  }

  int preimageSizeFZF(int k) { return help(k + 1) - help(k); }
};

int main() {
  assert(Solution().preimageSizeFZF(0) == 5);
  assert(Solution().preimageSizeFZF(5) == 0);
  assert(Solution().preimageSizeFZF(3) == 5);
}