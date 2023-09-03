/*
 * @Date: 2023-09-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-03
 * @FilePath: /algorithm/cpp/1921_eliminate_maximum/eliminate_maximum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int eliminateMaximum(vector<int>& dist, vector<int>& speed) {
    int n = dist.size();
    vector<int> arrivalTimes(n);
    for (int i = 0; i < n; i++) {
      arrivalTimes[i] = (dist[i] - 1) / speed[i] + 1;
    }
    sort(arrivalTimes.begin(), arrivalTimes.end());
    for (int i = 0; i < n; i++) {
      if (arrivalTimes[i] <= i) {
        return i;
      }
    }
    return n;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{1, 3, 4}, {1, 1, 1}, 3},
      {{1, 1, 2, 3}, {1, 1, 1, 1}, 1},
      {{3, 2, 4}, {5, 3, 2}, 1},
  };

  for (auto& [dist, speed, ans] : tests) {
    assert(Solution().eliminateMaximum(dist, speed) == ans);
  }
}