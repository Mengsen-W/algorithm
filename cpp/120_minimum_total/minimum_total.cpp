#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumTotal(vector<vector<int>>& triangle) {
    int n = triangle.size();
    vector<int> f(n);
    f[0] = triangle[0][0];
    for (int i = 1; i < n; ++i) {
      f[i] = f[i - 1] + triangle[i][i];
      for (int j = i - 1; j > 0; --j) {
        f[j] = min(f[j - 1], f[j]) + triangle[i][j];
      }
      f[0] += triangle[i][0];
    }
    return *min_element(f.begin(), f.end());
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{2}, {3, 4}, {6, 5, 7}, {4, 1, 8, 3}}, 11},
      {{{-10}}, -10},
  };

  for (auto& [triangle, ans] : tests) {
    assert(Solution().minimumTotal(triangle) == ans);
  }
}