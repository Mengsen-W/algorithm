#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int largestOverlap(vector<vector<int>>& A, vector<vector<int>>& B) {
    int N = A.size();
    vector<vector<int>> count(2 * N + 1, vector<int>(2 * N + 1, 0));

    for (int i = 0; i < N; ++i) {
      for (int j = 0; j < N; ++j) {
        if (A[i][j] == 1) {
          for (int i2 = 0; i2 < N; ++i2) {
            for (int j2 = 0; j2 < N; ++j2) {
              if (B[i2][j2] == 1) {
                count[i - i2 + N][j - j2 + N] += 1;
              }
            }
          }
        }
      }
    }

    int ans = 0;
    for (const auto& row : count) {
      for (int v : row) {
        ans = std::max(ans, v);
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>, int>> tests{
      {{{1, 1, 0}, {0, 1, 0}, {0, 1, 0}}, {{0, 0, 0}, {0, 1, 1}, {0, 0, 1}}, 3},
      {{{1}}, {{1}}, 1},
      {{{0}}, {{0}}, 0},
  };

  for (auto& [A, B, ans] : tests) {
    assert(Solution().largestOverlap(A, B) == ans);
  }
}