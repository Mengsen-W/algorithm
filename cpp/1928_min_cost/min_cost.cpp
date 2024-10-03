
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 private:
  // 极大值
  static constexpr int INFTY = INT_MAX / 2;

 public:
  int minCost(int maxTime, vector<vector<int>>& edges, vector<int>& passingFees) {
    int n = passingFees.size();
    vector<vector<int>> f(maxTime + 1, vector<int>(n, INFTY));
    f[0][0] = passingFees[0];
    for (int t = 1; t <= maxTime; ++t) {
      for (const auto& edge : edges) {
        int i = edge[0], j = edge[1], cost = edge[2];
        if (cost <= t) {
          f[t][i] = min(f[t][i], f[t - cost][j] + passingFees[i]);
          f[t][j] = min(f[t][j], f[t - cost][i] + passingFees[j]);
        }
      }
    }

    int ans = INFTY;
    for (int t = 1; t <= maxTime; ++t) {
      ans = min(ans, f[t][n - 1]);
    }
    return ans == INFTY ? -1 : ans;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>, int>> tests{
      {30, {{0, 1, 10}, {1, 2, 10}, {2, 5, 10}, {0, 3, 1}, {3, 4, 10}, {4, 5, 15}}, {5, 1, 2, 20, 20, 3}, 11},
      {29, {{0, 1, 10}, {1, 2, 10}, {2, 5, 10}, {0, 3, 1}, {3, 4, 10}, {4, 5, 15}}, {5, 1, 2, 20, 20, 3}, 48},
      {25, {{0, 1, 10}, {1, 2, 10}, {2, 5, 10}, {0, 3, 1}, {3, 4, 10}, {4, 5, 15}}, {5, 1, 2, 20, 20, 3}, -1},
  };


  for(auto &[maxTime, edges, passingFees, ans] : tests) {
    assert(Solution().minCost(maxTime, edges, passingFees) == ans);
  }
}