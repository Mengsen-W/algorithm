/*
 * @Date: 2022-04-24 09:47:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-24 09:54:39
 * @FilePath: /algorithm/868_binary_gap/binary_gap.cpp
 */

#include <algorithm>
#include <cassert>

using namespace std;

class Solution {
 public:
  int binaryGap(int n) {
    int last = -1, ans = 0;
    for (int i = 0; n; ++i) {
      if (n & 1) {
        if (last != -1) {
          ans = max(ans, i - last);
        }
        last = i;
      }
      n >>= 1;
    }
    return ans;
  }
};

int main() {
  assert(Solution().binaryGap(22) == 2);
  assert(Solution().binaryGap(8) == 5);
  assert(Solution().binaryGap(5) == 2);
}