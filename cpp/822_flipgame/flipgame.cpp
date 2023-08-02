/*
 * @Date: 2023-08-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-02
 * @FilePath: /algorithm/cpp/822_flipgame/flipgame.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int flipgame(vector<int>& fronts, vector<int>& backs) {
    int res = 3000, n = fronts.size();
    unordered_set<int> same;
    for (int i = 0; i < n; ++i) {
      if (fronts[i] == backs[i]) {
        same.insert(fronts[i]);
      }
    }
    for (int& x : fronts) {
      if (x < res && same.count(x) == 0) {
        res = x;
      }
    }
    for (int& x : backs) {
      if (x < res && same.count(x) == 0) {
        res = x;
      }
    }
    return res % 3000;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{1, 2, 4, 4, 7}, {1, 3, 4, 1, 3}, 2},
  };

  for (auto& [fronts, backs, ans] : tests) {
    assert(Solution().flipgame(fronts, backs) == ans);
  }
}