/*
 * @Date: 2021-11-09 01:24:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-09 02:06:30
 */

#include <cassert>
#include <set>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int min_cnt = 0x3f3f3f3f;
  set<pair<string, int>> vis;
  void processBoard(string& board) {
    int board_size = board.size();
    for (int slow = 0, fast = 0; fast <= board_size; fast++) {
      int board_size_cur = board.size();
      if (fast < board_size_cur && board[slow] == board[fast]) continue;
      if (fast - slow >= 3) {
        board.erase(slow, fast - slow);
        fast = 0;
      }
      slow = fast;
    }
  }
  void helper(string& board, string& hand, int cnt) {
    if (cnt >= min_cnt) return;
    if (vis.count({board, cnt})) return;
    vis.insert({board, cnt});
    if (board.empty()) {
      min_cnt = min(min_cnt, cnt);
      return;
    }
    if (hand.empty()) return;
    int n = board.size(), m = hand.size();
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < m; ++j) {
        char ch = hand[j];
        hand.erase(hand.begin() + j);
        // new board
        string nb = board;
        nb.insert(nb.begin() + i, ch);
        processBoard(nb);
        helper(nb, hand, cnt + 1);
        hand.insert(hand.begin() + j, ch);
      }
    }
  }
  int findMinStep(string board, string hand) {
    helper(board, hand, 0);
    return min_cnt == 0x3f3f3f3f ? -1 : min_cnt;
  }
};

int main() {
  assert(Solution().findMinStep("WRRBBW", "RB") == -1);
  assert(Solution().findMinStep("WWRRBBWW", "WRBRW") == 2);
  assert(Solution().findMinStep("G", "GGGGG") == 2);
  assert(Solution().findMinStep("RBYYBBRRB", "YRBGB") == 3);
}