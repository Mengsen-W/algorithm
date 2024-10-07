#include <cassert>
#include <queue>
#include <tuple>
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
  vector<tuple<int, int, vector<vector<int>>, int>> tests{
      {1, 1, {}, 0},
      {100, 1, {{10, 100}}, -1},
      {100, 10, {{10, 60}, {20, 30}, {30, 30}, {60, 40}}, 2},
  };

  for (auto &[target, startFuel, stations, ans] : tests) {
    assert(Solution().minRefuelStops(target, startFuel, stations) == ans);
  }
}