/*
 * @Date: 2023-12-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-27
 * @FilePath: /algorithm/cpp/2660_is_winner/is_winner.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int score(const vector<int>& player) {
    int res = 0;
    for (int i = 0; i < player.size(); i++) {
      if ((i > 0 && player[i - 1] == 10) || (i > 1 && player[i - 2] >= 10)) {
        res += 2 * player[i];
      } else {
        res += player[i];
      }
    }
    return res;
  }

  int isWinner(vector<int>& player1, vector<int>& player2) {
    int s1 = score(player1);
    int s2 = score(player2);
    return s1 == s2 ? 0 : s1 > s2 ? 1 : 2;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{4, 10, 7, 9}, {6, 5, 2, 3}, 1},
      {{3, 5, 7, 6}, {8, 10, 10, 2}, 2},
      {{2, 3}, {4, 1}, 0},
  };

  for (auto& [player1, player2, ans] : tests) {
    assert(Solution().isWinner(player1, player2) == ans);
  }
}