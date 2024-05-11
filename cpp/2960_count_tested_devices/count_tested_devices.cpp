/*
 * @Date: 2024-05-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-10
 * @FilePath: /algorithm/cpp/2960_count_tested_devices/count_tested_devices.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countTestedDevices(vector<int>& batteryPercentages) {
    int need = 0;
    for (int battery : batteryPercentages) {
      if (battery > need) {
        need++;
      }
    }
    return need;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 1, 2, 1, 3}, 3},
      {{0, 1, 2}, 2},
  };

  for (auto& [batteryPercentages, ans] : tests) {
    assert(Solution().countTestedDevices(batteryPercentages) == ans);
  }
}