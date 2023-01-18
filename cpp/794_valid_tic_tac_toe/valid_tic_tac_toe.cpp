/*
 * @Date: 2021-12-09 05:48:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-09 06:06:02
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  bool validTicTacToe(vector<string> board) {
    int xCount = 0, oCount = 0;
    for (string& row : board) {
      for (char c : row) {
        xCount = (c == 'X') ? (xCount + 1) : xCount;
        oCount = (c == 'O') ? (oCount + 1) : oCount;
      }
    }
    if (oCount != xCount && oCount != xCount - 1) return false;
    if (win(board, 'X') && oCount != xCount - 1) return false;
    if (win(board, 'O') && oCount != xCount) return false;
    return true;
  }

  bool win(vector<string>& board, char p) {
    for (int i = 0; i < 3; ++i) {
      if (p == board[i][0] && p == board[i][1] && p == board[i][2]) return true;
      if (p == board[0][i] && p == board[1][i] && p == board[2][i]) return true;
    }
    if (p == board[0][0] && p == board[1][1] && p == board[2][2]) return true;
    if (p == board[0][2] && p == board[1][1] && p == board[2][0]) return true;
    return false;
  }
};

int main() {
  assert(!Solution().validTicTacToe(vector<string>{"O  ", "   ", "   "}));
  assert(!Solution().validTicTacToe(vector<string>{"XOX", " X ", "   "}));
  assert(!Solution().validTicTacToe(vector<string>{"XXX", "   ", "OOO"}));
  assert(Solution().validTicTacToe(vector<string>{"XOX", "O O", "XOX"}));
}
