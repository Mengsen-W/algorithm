/*
 * @Date: 2023-08-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-30
 * @FilePath: /algorithm/cpp/1654_minimum_jumps/minimum_jumps.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumJumps(vector<int>& forbidden, int a, int b, int x) {
    queue<tuple<int, int, int>> q;
    unordered_set<int> visited;
    q.emplace(0, 1, 0);
    visited.emplace(0);
    int lower = 0, upper = max(*max_element(forbidden.begin(), forbidden.end()) + a, x) + b;
    unordered_set<int> forbiddenSet(forbidden.begin(), forbidden.end());
    while (!q.empty()) {
      auto [position, direction, step] = q.front();
      q.pop();
      if (position == x) {
        return step;
      }
      int nextPosition = position + a;
      int nextDirection = 1;
      if (lower <= nextPosition && nextPosition <= upper && !visited.count(nextPosition * nextDirection) &&
          !forbiddenSet.count(nextPosition)) {
        visited.emplace(nextPosition * nextDirection);
        q.emplace(nextPosition, nextDirection, step + 1);
      }
      if (direction == 1) {
        nextPosition = position - b;
        nextDirection = -1;
        if (lower <= nextPosition && nextPosition <= upper && !visited.count(nextPosition * nextDirection) &&
            !forbiddenSet.count(nextPosition)) {
          visited.emplace(nextPosition * nextDirection);
          q.emplace(nextPosition, nextDirection, step + 1);
        }
      }
    }
    return -1;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, int, int>> tests{
      {{14, 4, 18, 1, 15}, 3, 15, 9, 3},
      {{8, 3, 16, 6, 12, 20}, 15, 13, 11, -1},
      {{1, 6, 2, 14, 5, 17, 4}, 16, 9, 7, 2},
  };

  for (auto& [forbidden, a, b, x, expected] : tests) {
    assert(Solution().minimumJumps(forbidden, a, b, x) == expected);
  }
}