#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxPointsInsideSquare(vector<vector<int>>& points, string s) {
    vector<int> min1(26, 1000000001);
    int min2 = 1000000001;
    int n = s.length();
    for (int i = 0; i < n; ++i) {
      int x = points[i][0], y = points[i][1], j = s[i] - 'a';
      int d = max(abs(x), abs(y));
      if (d < min1[j]) {
        min2 = min(min2, min1[j]);
        min1[j] = d;
      } else if (d < min2) {
        min2 = d;
      }
    }
    int res = 0;
    for (int d : min1) {
      if (d < min2) {
        ++res;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, string, int>> tests{
      {{{2, 2}, {-1, -2}, {-4, 4}, {-3, 1}, {3, -3}}, "abdca", 2},
      {{{1, 1}, {-1, -1}, {2, -2}}, "ccd", 0},
      {{{1, 1}, {-2, -2}, {-2, 2}}, "abb", 1},
  };

  for (auto& [points, s, ans] : tests) {
    assert(Solution().maxPointsInsideSquare(points, s) == ans);
  }
}