#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxValueOfCoins(vector<vector<int>>& piles, int k) {
    vector<int> f(k + 1, -1);
    f[0] = 0;
    for (const auto& pile : piles) {
      for (int i = k; i > 0; --i) {
        int value = 0;
        for (int t = 1; t <= pile.size(); ++t) {
          value += pile[t - 1];
          if (i >= t && f[i - t] != -1) {
            f[i] = max(f[i], f[i - t] + value);
          }
        }
      }
    }
    return f[k];
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {{{1, 100, 3}, {7, 8, 9}}, 2, 101},
      {{{100}, {100}, {100}, {100}, {100}, {100}, {1, 1, 1, 1, 1, 1, 700}}, 7, 706},
  };
  
  for (auto &[piles, k, ans] : tests) {
    assert(Solution().maxValueOfCoins(piles, k) == ans);
  }
}