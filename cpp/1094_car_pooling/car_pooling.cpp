/*
 * @Date: 2023-12-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-02
 * @FilePath: /algorithm/cpp/1094_car_pooling/car_pooling.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool carPooling(vector<vector<int>>& trips, int capacity) {
    int to_max = 0;
    for (const auto& trip : trips) {
      to_max = max(to_max, trip[2]);
    }

    vector<int> diff(to_max + 1);
    for (const auto& trip : trips) {
      diff[trip[1]] += trip[0];
      diff[trip[2]] -= trip[0];
    }

    int count = 0;
    for (int i = 0; i <= to_max; ++i) {
      count += diff[i];
      if (count > capacity) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, bool>> tests{
      {{{2, 1, 5}, {3, 3, 7}}, 4, false},
      {{{2, 1, 5}, {3, 3, 7}}, 5, true},
  };

  for (auto& [trips, capacity, ans] : tests) {
    assert(Solution().carPooling(trips, capacity) == ans);
  }
}