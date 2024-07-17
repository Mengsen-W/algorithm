#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfSets(int n, int maxDistance, vector<vector<int>>& roads) {
    int res = 0;
    vector<int> opened(n, 0);
    vector<vector<int>> d(n, vector<int>(n, 1000000));

    for (int mask = 0; mask < (1 << n); ++mask) {
      for (int i = 0; i < n; ++i) {
        opened[i] = mask & (1 << i);
      }
      fill(d.begin(), d.end(), vector<int>(n, 1000000));
      for (const auto& road : roads) {
        int i = road[0], j = road[1], r = road[2];
        if (opened[i] > 0 && opened[j] > 0) {
          d[i][j] = d[j][i] = min(d[i][j], r);
        }
      }

      // Floyd-Warshall algorithm
      for (int k = 0; k < n; ++k) {
        if (opened[k] > 0) {
          for (int i = 0; i < n; ++i) {
            if (opened[i] > 0) {
              for (int j = i + 1; j < n; ++j) {
                if (opened[j] > 0) {
                  d[i][j] = d[j][i] = min(d[i][j], d[i][k] + d[k][j]);
                }
              }
            }
          }
        }
      }

      // Validate
      int good = 1;
      for (int i = 0; i < n; ++i) {
        if (opened[i] > 0) {
          for (int j = i + 1; j < n; ++j) {
            if (opened[j] > 0 && d[i][j] > maxDistance) {
              good = 0;
              break;
            }
          }
          if (!good) {
            break;
          }
        }
      }

      res += good;
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int, vector<vector<int>>, int>> tests{
      {3, 5, {{0, 1, 2}, {1, 2, 10}, {0, 2, 10}}, 5},
      {3, 5, {{0, 1, 20}, {0, 1, 10}, {1, 2, 2}, {0, 2, 2}}, 7},
  };

  for (auto& [n, maxDistance, roads, ans] : tests) {
    assert(Solution().numberOfSets(n, maxDistance, roads) == ans);
  }
}