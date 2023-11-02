/*
 * @Date: 2023-11-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-02
 * @FilePath: /algorithm/cpp/2103_count_points/count_points.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  static constexpr int POLE_NUM = 10;
  int countPoints(string rings) {
    vector<int> state(POLE_NUM);
    int n = rings.size();
    for (int i = 0; i < n; i += 2) {
      char color = rings[i];
      int pole_index = rings[i + 1] - '0';
      if (color == 'R') {
        state[pole_index] |= 1;
      } else if (color == 'G') {
        state[pole_index] |= 2;
      } else {
        state[pole_index] |= 4;
      }
    }
    int res = 0;
    for (int i = 0; i < POLE_NUM; i++) {
      res += state[i] == 7;
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"B0B6G0R6R0R6G9", 1},
      {"B0R0G0R9R0B0G0", 1},
      {"G4", 0},
  };

  for (auto &[rings, ans] : tests) {
    assert(Solution().countPoints(rings) == ans);
  }
}