/*
 * @Date: 2022-07-04
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-04
 * @FilePath: /algorithm/871_min_refuel_stops/min_refuel_stops.cpp
 */

#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  int minRefuelStops(int target, int startFuel, vector<vector<int>> stations) {
    priority_queue<int> pq;
    int ans = 0, prev = 0, fuel = startFuel;
    int n = stations.size();
    for (int i = 0; i <= n; i++) {
      int curr = i < n ? stations[i][0] : target;
      fuel -= curr - prev;
      while (fuel < 0 && !pq.empty()) {
        fuel += pq.top();
        pq.pop();
        ans++;
      }
      if (fuel < 0) {
        return -1;
      }
      if (i < n) {
        pq.emplace(stations[i][1]);
        prev = curr;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> stations{};
    assert(Solution().minRefuelStops(1, 1, stations) == 0);
  }

  {
    vector<vector<int>> stations{{10, 100}};
    assert(Solution().minRefuelStops(100, 1, stations) == -1);
  }

  {

    vector<vector<int>> stations{{10,60},{20,30},{30,30},{60,40}};
    assert(Solution().minRefuelStops(100, 10, stations) == 2);
  }
}