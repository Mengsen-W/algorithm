/*
 * @Date: 2024-05-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-22
 * @FilePath: /algorithm/cpp/2225_find_winners/find_winners.cpp
 */

#include <algorithm>
#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> findWinners(vector<vector<int>>& matches) {
    unordered_map<int, int> freq;
    for (const auto& match : matches) {
      int winner = match[0], loser = match[1];
      if (!freq.count(winner)) {
        freq[winner] = 0;
      }
      ++freq[loser];
    }

    vector<vector<int>> ans(2);
    for (const auto& [key, value] : freq) {
      if (value < 2) {
        ans[value].push_back(key);
      }
    }

    sort(ans[0].begin(), ans[0].end());
    sort(ans[1].begin(), ans[1].end());
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>>> tests{
      {{{1, 3}, {2, 3}, {3, 6}, {5, 6}, {5, 7}, {4, 5}, {4, 8}, {4, 9}, {10, 4}, {10, 9}}, {{1, 2, 10}, {4, 5, 7, 8}}},
      {{{2, 3}, {1, 3}, {5, 4}, {6, 4}}, {{1, 2, 5, 6}, {}}},
  };

  for (auto& [mathes, ans] : tests) {
    assert(Solution().findWinners(mathes) == ans);
  }
}
