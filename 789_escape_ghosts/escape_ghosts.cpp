/*
 * @Date: 2021-08-22 13:17:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-22 13:37:41
 */

#include <cassert>
#include <cmath>
#include <functional>
#include <vector>
using namespace std;

class Solution {
 public:
  bool escapeGhosts(vector<vector<int>>& ghosts, vector<int>& target) {
    vector<int> source(2);
    function<int(vector<int>&, vector<int>&)> manhattanDistance =
        [](vector<int>& point1, vector<int>& point2) -> int {
      return abs(point1[0] - point2[0]) + abs(point1[1] - point2[1]);
    };
    int distance = manhattanDistance(source, target);
    for (auto& ghost : ghosts) {
      int ghostDistance = manhattanDistance(ghost, target);
      if (ghostDistance <= distance) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  {
    vector<vector<int>> ghosts{{1, 0}, {0, 3}};
    vector<int> target{0, 1};
    assert(Solution{}.escapeGhosts(ghosts, target));
  }
  {
    vector<vector<int>> ghosts{{1, 0}};
    vector<int> target{2, 0};
    assert(!Solution{}.escapeGhosts(ghosts, target));
  }
  {
    vector<vector<int>> ghosts{{2, 0}};
    vector<int> target{1, 0};
    assert(!Solution{}.escapeGhosts(ghosts, target));
  }
  {
    vector<vector<int>> ghosts{{5, 0}, {-10, -2}, {0, -5}, {-2, -2}, {-7, 1}};
    vector<int> target{7, 7};
    assert(!Solution{}.escapeGhosts(ghosts, target));
  }
  {
    vector<vector<int>> ghosts{{-1, 0}, {0, 1}, {-1, 0}, {0, 1}, {-1, 0}};
    vector<int> target{0, 0};
    assert(Solution{}.escapeGhosts(ghosts, target));
  }
}