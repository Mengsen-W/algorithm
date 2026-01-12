#include <cassert>
#include <cstdlib>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    int minTimeToVisitAllPoints(vector<vector<int>>& points) {
        int x0 = points[0][0], x1 = points[0][1];
        int ans = 0;
        for (int i = 1; i < points.size(); ++i) {
            int y0 = points[i][0], y1 = points[i][1];
            ans += max(abs(x0 - y0), abs(x1 - y1));
            x0 = y0;
            x1 = y1;
        }
        return ans;
    }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 1}, {3, 4}, {-1, 0}}, 7},
      {{{3, 2}, {-2, 2}}, 5},
  };

  for (auto &[points, ans] : tests) {
    assert(Solution().minTimeToVisitAllPoints(points) == ans);
  }
}