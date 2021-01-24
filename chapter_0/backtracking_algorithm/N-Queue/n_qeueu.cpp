/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-20 16:54:53
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-22 17:09:58
 */

#include <iostream>
#include <string>
#include <vector>

using namespace std;

vector<vector<string>> solveNQueens(int n);
void backtrack(vector<string> &board, int row);
bool is_valid(vector<string> &board, int row, int col);

vector<vector<string>> res;

vector<vector<string>> solveNQueens(int n) {
  // '.' 表⽰空，'Q' 表⽰皇后，初始化空棋盘。 vector<string>
  vector<string> board(n, string(n, '.'));
  backtrack(board, 0);
  return res;
}

// 路径：board中小于row的那些行已经成功放置了皇后
// 选择列表：第row行的所有列都放置皇后的选择
// 结束条件：row超过board的最后一行
void backtrack(vector<string> &board, int row) {
  // 触发结束条件
  if (row == board.size()) {
    res.push_back(board);
    return;
    // return true;
  }
  int n = board[row].size();

  for (int col = 0; col < n; ++col) {
    if (!is_valid(board, row, col)) continue;
    // 做选择
    board[row][col] = 'Q';
    // 进入下一行决策
    backtrack(board, row + 1);
    // if(backtrack(board, row + 1)) return true;
    // 撤销选择
    board[row][col] = '.';
  }
  // return false;
}

// 是否可以放置皇后
bool is_valid(vector<string> &board, int row, int col) {
  int n = board.size();

  // 检查列是否有冲突
  for (int i = 0; i < n; ++i)
    if (board[i][col] == 'Q') return false;

  // 检查右上方是否有冲突
  for (int i = row - 1, j = col + 1; i >= 0 && j < n; --i, ++j)
    if (board[i][j] == 'Q') return false;

  // 检查左上方是否有冲突
  for (int i = row - 1, j = col - 1; i >= 0 && j >= 0; --i, --j)
    if (board[i][j] == 'Q') return false;

  return true;
}

int main() {
  solveNQueens(4);

  for (int i = 0; i < res.size(); ++i) {
    for (int j = 0; j < res[0].size(); ++j) {
      cout << res[i][j] << endl;
    }
    cout << endl;
  }
  return 0;
}