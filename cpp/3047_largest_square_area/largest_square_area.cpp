#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  long long largestSquareArea(vector<vector<int>>& bottomLeft, vector<vector<int>>& topRight) {
    int n = bottomLeft.size();
    int maxSide = 0;

    for (int i = 0; i < n; i++) {
      for (int j = i + 1; j < n; j++) {
        int w = min(topRight[i][0], topRight[j][0]) - max(bottomLeft[i][0], bottomLeft[j][0]);
        int h = min(topRight[i][1], topRight[j][1]) - max(bottomLeft[i][1], bottomLeft[j][1]);

        maxSide = max(maxSide, min(w, h));
      }
    }

    return 1LL * maxSide * maxSide;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>, long long>> tests{
      {{{1, 1}, {2, 2}, {3, 1}}, {{3, 3}, {4, 4}, {6, 6}}, 1},
      {{{1, 1}, {2, 2}, {1, 2}}, {{3, 3}, {4, 4}, {3, 4}}, 1},
      {{{1, 1}, {3, 3}, {3, 1}}, {{2, 2}, {4, 4}, {4, 2}}, 0},
  };

  for (auto [bottomLeft, topRight, expected] : tests) {
    assert(Solution().largestSquareArea(bottomLeft, topRight) == expected);
  }
}