/*
 * @Date: 2024-01-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-28
 * @FilePath: /algorithm/cpp/365_can_measure_water/can_measure_water.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  bool canMeasureWater(int x, int y, int z) {
    if (x + y < z) {
      return false;
    }
    if (x == 0 || y == 0) {
      return z == 0 || x + y == z;
    }
    return z % gcd(x, y) == 0;
  }
};

int main() {
  vector<tuple<int, int, int, bool>> tests{
      {3, 5, 4, true},
      {2, 6, 5, false},
      {1, 2, 3, true},
  };

  for (auto& [x, y, z, ans] : tests) {
    assert(Solution().canMeasureWater(x, y, z) == ans);
  }
}