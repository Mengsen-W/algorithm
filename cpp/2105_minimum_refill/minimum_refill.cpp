/*
 * @Date: 2024-05-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-09
 * @FilePath: /algorithm/cpp/2105_minimum_refill/minimum_refill.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumRefill(vector<int>& plants, int capacityA, int capacityB) {
    int res = 0;  // 灌满水罐次数
    int n = plants.size();
    int posa = 0, posb = n - 1;              // 两人位置
    int vala = capacityA, valb = capacityB;  // 两人剩余水量
    // 模拟相遇前的浇水过程
    while (posa < posb) {
      if (vala < plants[posa]) {
        ++res;
        vala = capacityA - plants[posa];
      } else {
        vala -= plants[posa];
      }
      ++posa;
      if (valb < plants[posb]) {
        ++res;
        valb = capacityB - plants[posb];
      } else {
        valb -= plants[posb];
      }
      --posb;
    }
    // 模拟相遇后可能的浇水过程
    if (posa == posb) {
      if (vala >= valb && vala < plants[posa]) {
        ++res;
      }
      if (vala < valb && valb < plants[posb]) {
        ++res;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, int>> tests{
      {{2, 2, 3, 3}, 5, 5, 1},
      {{2, 2, 3, 3}, 3, 4, 2},
      {{5}, 10, 8, 0},
  };

  for (auto& [plants, capacityA, capacityB, ans] : tests) {
    assert(Solution().minimumRefill(plants, capacityA, capacityB) == ans);
  }
}