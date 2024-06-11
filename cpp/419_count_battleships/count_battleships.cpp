/*
 * @Date: 2021-12-18 01:04:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-18 01:37:12
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countBattleships(vector<vector<char>>& board) {
    int row = board.size();
    int col = board[0].size();
    int ans = 0;
    for (int i = 0; i < row; ++i) {
      for (int j = 0; j < col; ++j) {
        if (board[i][j] == 'X') {
          if (i > 0 && board[i - 1][j] == 'X') {
            continue;
          }
          if (j > 0 && board[i][j - 1] == 'X') {
            continue;
          }
          ans++;
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<char>>, int>> tests{
      {{{'X', '.', '.', 'X'}, {'.', '.', '.', 'X'}, {'.', '.', '.', 'X'}}, 2},
      {{{'.'}}, 0},
  };

  for (auto& [board, ans] : tests) {
    assert(Solution().countBattleships(board) == ans);
  }
}