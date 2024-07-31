#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minRectanglesToCoverPoints(vector<vector<int>>& points, int w) {
    sort(points.begin(), points.end());
    int res = 0;
    int bound = -1;
    for (auto& p : points) {
      if (p[0] > bound) {
        bound = p[0] + w;
        res++;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {{{2, 1}, {1, 0}, {1, 4}, {1, 8}, {3, 5}, {4, 6}}, 1, 2},
      {{{0, 0}, {1, 1}, {2, 2}, {3, 3}, {4, 4}, {5, 5}, {6, 6}}, 2, 3},
      {{{2, 3}, {1, 2}}, 0, 2},
  };

  for (auto& [points, w, ans] : tests) {
    assert(Solution().minRectanglesToCoverPoints(points, w) == ans);
  }
}