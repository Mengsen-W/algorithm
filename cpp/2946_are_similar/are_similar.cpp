#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool areSimilar(vector<vector<int>>& mat, int k) {
    int m = mat.size(), n = mat[0].size();
    k %= n;

    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        if (mat[i][j] != mat[i][(j + k) % n]) {
          return false;
        }
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, bool>> tests{
      {{{1, 2, 1, 2}, {5, 5, 5, 5}, {6, 3, 6, 3}}, 2, true},
      {{{2, 2}, {2, 2}}, 3, true},
      {{{1, 2}}, 1, false},
  };

  for (auto& [mat, k, expected] : tests) {
    assert(Solution().areSimilar(mat, k) == expected);
  }
}