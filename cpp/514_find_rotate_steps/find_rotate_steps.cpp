/*
 * @Date: 2024-01-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-29
 * @FilePath: /algorithm/cpp/514_find_rotate_steps/find_rotate_steps.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findRotateSteps(string ring, string key) {
    int n = ring.size(), m = key.size();
    vector<int> pos[26];
    for (int i = 0; i < n; ++i) {
      pos[ring[i] - 'a'].push_back(i);
    }
    vector<vector<int>> dp(m, vector<int>(n, 0x3f3f3f3f));
    for (auto& i : pos[key[0] - 'a']) {
      dp[0][i] = min(i, n - i) + 1;
    }
    for (int i = 1; i < m; ++i) {
      for (auto& j : pos[key[i] - 'a']) {
        for (auto& k : pos[key[i - 1] - 'a']) {
          dp[i][j] = min(dp[i][j], dp[i - 1][k] + min(abs(j - k), n - abs(j - k)) + 1);
        }
      }
    }
    return *min_element(dp[m - 1].begin(), dp[m - 1].end());
  }
};

int main() {
  vector<tuple<string, string, int>> tests{
      {"godding", "gd", 4},
      {"godding", "godding", 13},
  };

  for (auto& [ring, key, ans] : tests) {
    assert(Solution().findRotateSteps(ring, key) == ans);
  }
}
