/*
 * @Date: 2024-05-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-08
 * @FilePath: /algorithm/cpp/2097_watering_plants/watering_plants.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int wateringPlants(vector<int>& plants, int capacity) {
    int n = plants.size();
    int ans = 0;
    int rest = capacity;
    for (int i = 0; i < n; ++i) {
      if (rest >= plants[i]) {
        ++ans;
        rest -= plants[i];
      } else {
        ans += i * 2 + 1;
        rest = capacity - plants[i];
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{2, 2, 3, 3}, 5, 14},
      {{1, 1, 1, 4, 2, 3}, 4, 30},
      {{7, 7, 7, 7, 7, 7, 7}, 8, 49},
  };

  for (auto& [plants, capacity, ans] : tests) {
    assert(Solution().wateringPlants(plants, capacity) == ans);
  }
}