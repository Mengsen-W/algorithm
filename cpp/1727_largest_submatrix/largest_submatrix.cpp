#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int largestSubmatrix(vector<vector<int>>& matrix) {
    int m = matrix.size(), n = matrix[0].size();
    int maxArea = 0;
    for (int i = 1; i < m; i++) {
      for (int j = 0; j < n; j++) {
        if (matrix[i][j] == 1) {
          matrix[i][j] += matrix[i - 1][j];
        }
      }
    }
    for (int i = 0; i < m; i++) {
      sort(matrix[i].begin(), matrix[i].end(), greater<int>());
      for (int j = 0; j < n; j++) {
        maxArea = max(maxArea, (j + 1) * matrix[i][j]);
      }
    }
    return maxArea;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{0, 0, 1}, {1, 1, 1}, {1, 0, 1}}, 4},
      {{{1, 0, 1, 0, 1}}, 3},
      {{{1, 1, 0}, {1, 0, 1}}, 2},
      {{{0, 0}, {0, 0}}, 0},
  };

  for (auto [matrix, expected] : tests) {
    assert(Solution().largestSubmatrix(matrix) == expected);
  }
}