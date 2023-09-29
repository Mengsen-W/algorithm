/*
 * @Date: 2023-09-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-29
 * @FilePath: /algorithm/cpp/605_can_place_flowers/can_place_flowers.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canPlaceFlowers(vector<int>& flowerbed, int n) {
    int count = 0;
    int m = flowerbed.size();
    int prev = -1;
    for (int i = 0; i < m; i++) {
      if (flowerbed[i] == 1) {
        if (prev < 0) {
          count += i / 2;
        } else {
          count += (i - prev - 2) / 2;
        }
        if (count >= n) {
          return true;
        }
        prev = i;
      }
    }
    if (prev < 0) {
      count += (m + 1) / 2;
    } else {
      count += (m - prev - 1) / 2;
    }
    return count >= n;
  }
};

int main() {
  vector<tuple<vector<int>, int, bool>> tests{
      {{1, 0, 0, 0, 1}, 1, true},
      {{1, 0, 0, 0, 1}, 2, false},
  };

  for (auto& [flowerbed, n, expected] : tests) {
    assert(Solution().canPlaceFlowers(flowerbed, n) == expected);
  }
}