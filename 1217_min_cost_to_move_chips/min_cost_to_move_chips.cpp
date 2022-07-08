/*
 * @Date: 2022-07-08
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-08
 * @FilePath: /algorithm/1217_min_cost_to_move_chips/min_cost_to_move_chips.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minCostToMoveChips(vector<int> position) {
    int even = 0, odd = 0;
    for (int pos : position) {
      if (pos % 2) {
        odd++;
      } else {
        even++;
      }
    }
    return min(odd, even);
  }
};

int main() {
  assert(Solution().minCostToMoveChips({1, 2, 3}) == 1);
  assert(Solution().minCostToMoveChips({2, 2, 2, 3, 3}) == 2);
  assert(Solution().minCostToMoveChips({1, 10000000}) == 1);
}