/*
 * @Date: 2023-09-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-08
 * @FilePath: /algorithm/cpp/2651_find_delayed_arrival_time/find_delayed_arrival_time.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int findDelayedArrivalTime(int arrivalTime, int delayedTime) { return (arrivalTime + delayedTime) % 24; }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {15, 5, 20},
      {13, 11, 0},
  };

  for (auto &[arrivalTime, delayedTime, expected] : tests) {
    assert(Solution().findDelayedArrivalTime(arrivalTime, delayedTime) == expected);
  }
}