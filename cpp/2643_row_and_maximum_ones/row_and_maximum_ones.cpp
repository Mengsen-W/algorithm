#include <cassert>
#include <functional>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> rowAndMaximumOnes(vector<vector<int>>& mat) {
    int maxOnes = 0, rowIndex = 0;
    for (int i = 0; i < mat.size(); i++) {
      int tot = accumulate(mat[i].begin(), mat[i].end(), 0);
      if (tot > maxOnes) {
        maxOnes = tot;
        rowIndex = i;
      }
    }
    return {rowIndex, maxOnes};
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>>> tests{
      {{{0, 1}, {1, 0}}, {0, 1}},
      {{{0, 0, 0}, {0, 1, 1}}, {1, 2}},
      {{{0, 0}, {1, 1}, {0, 0}}, {1, 2}},
  };

  for (auto& [mat, ans] : tests) {
    assert(Solution().rowAndMaximumOnes(mat) == ans);
  }
}