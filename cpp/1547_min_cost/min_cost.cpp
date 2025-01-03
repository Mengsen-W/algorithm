#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minCost(int n, vector<int>& cuts) {
    int m = cuts.size();
    sort(cuts.begin(), cuts.end());
    cuts.insert(cuts.begin(), 0);
    cuts.push_back(n);
    vector<vector<int>> f(m + 2, vector<int>(m + 2));
    for (int i = m; i >= 1; --i) {
      for (int j = i; j <= m; ++j) {
        f[i][j] = (i == j ? 0 : INT_MAX);
        for (int k = i; k <= j; ++k) {
          f[i][j] = min(f[i][j], f[i][k - 1] + f[k + 1][j]);
        }
        f[i][j] += cuts[j + 1] - cuts[i - 1];
      }
    }
    return f[1][m];
  }
};

int main() {
  vector<tuple<int, vector<int>, int>> tests{
      {7, {1, 3, 4, 5}, 16},
      {9, {5, 6, 1, 4, 2}, 22},
  };

  for (auto &[n, cuts, ans] : tests) {
    assert(Solution().minCost(n, cuts) == ans);
  }
}