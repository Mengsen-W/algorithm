#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int winningPlayerCount(int n, vector<vector<int>>& pick) {
    vector<vector<int>> cnt(n, vector<int>(11));
    for (auto& p : pick) {
      cnt[p[0]][p[1]]++;
    }
    int ans = 0;
    for (int i = 0; i < n; i++) {
      for (int j = 0; j <= 10; j++) {
        if (cnt[i][j] > i) {
          ans++;
          break;
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int>> tests{
      {4, {{0, 0}, {1, 0}, {1, 0}, {2, 1}, {2, 1}, {2, 0}}, 2},
      {5, {{1, 1}, {1, 2}, {1, 3}, {1, 4}}, 0},
      {5, {{1, 1}, {2, 4}, {2, 4}, {2, 4}}, 1},
  };

  for (auto& [n, pick, ans] : tests) {
    assert(Solution().winningPlayerCount(n, pick) == ans);
  }
}