/*
 * @Date: 2021-11-07 02:02:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-07 02:17:42
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxCount(int m, int n, vector<vector<int>> ops) {
    int mina = m, minb = n;
    for (const auto& op : ops) {
      mina = min(mina, op[0]);
      minb = min(minb, op[1]);
    }
    return mina * minb;
  }
};

int main() {
  assert(Solution().maxCount(3, 3, {{2, 2}, {3, 3}}) == 4);
  return 0;
}