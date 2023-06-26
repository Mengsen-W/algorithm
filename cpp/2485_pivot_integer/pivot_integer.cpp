/*
 * @Date: 2023-06-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-26
 * @FilePath: /algorithm/cpp/2485_pivot_integer/pivot_integer.cpp
 */

#include <cassert>
#include <cmath>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  int pivotInteger(int n) {
    int t = (n * n + n) / 2;
    int x = sqrt(t);
    if (x * x == t) {
      return x;
    }
    return -1;
  }
};

int main() {
  unordered_map<int, int> testMap{
      {8, 6},
      {1, 1},
      {4, -1},
  };
  for (const auto& [key, value] : testMap) {
    assert(Solution().pivotInteger(key) == value);
  }
}
