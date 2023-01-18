/*
 * @Date: 2022-07-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-24
 * @FilePath: /algorithm/1184_distance_between_bus_stops/distance_between_bus_stops.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int distanceBetweenBusStops(vector<int>& distance, int start, int destination) {
    if (start > destination) {
      swap(start, destination);
    }
    return min(accumulate(distance.begin() + start, distance.begin() + destination, 0),
               accumulate(distance.begin(), distance.begin() + start, 0) +
                   accumulate(distance.begin() + destination, distance.end(), 0));
  }
};

int main() {
  {
    vector<int> distance{1, 2, 3, 4};
    int start = 0;
    int destination = 1;
    assert(Solution().distanceBetweenBusStops(distance, start, destination) == 1);
  }

  {
    vector<int> distance{1, 2, 3, 4};
    int start = 0;
    int destination = 2;
    assert(Solution().distanceBetweenBusStops(distance, start, destination) == 3);
  }

  {
    vector<int> distance{1, 2, 3, 4};
    int start = 0;
    int destination = 3;
    assert(Solution().distanceBetweenBusStops(distance, start, destination) == 4);
  }
}
