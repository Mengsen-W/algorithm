/*
 * @Date: 2023-08-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-16
 * @FilePath: /algorithm/cpp/2682_circular_game_losers/circular_game_losers.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> circularGameLosers(int n, int k) {
    vector<bool> visit(n, false);
    for (int i = k, j = 0; !visit[j]; i += k) {
      visit[j] = true;
      j = (j + i) % n;
    }
    vector<int> ans;
    for (int i = 0; i < n; i++) {
      if (!visit[i]) {
        ans.emplace_back(i + 1);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, int, vector<int>>> test_cases{
      {5, 2, {4, 5}},
      {4, 4, {2, 3, 4}},
  };

  for (auto &[n, k, ans] : test_cases) {
    assert(Solution().circularGameLosers(n, k) == ans);
  }
}