#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int stoneGameVIII(vector<int>& stones) {
    int n = stones.size();
    vector<int> pre;
    partial_sum(stones.begin(), stones.end(), back_inserter(pre));
    vector<int> f(n);
    f[n - 1] = pre[n - 1];
    for (int i = n - 2; i >= 1; --i) {
      f[i] = max(f[i + 1], pre[i] - f[i + 1]);
    }
    return f[1];
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{-1, 2, -3, 4, -5}, 5},
      {{7, -6, 5, 10, 5, -2, -6}, 13},
      {{-10, -12}, -22},
  };

  for (auto [stones, expected] : tests) {
    assert(Solution().stoneGameVIII(stones) == expected);
  }
}