/*
 * @Date: 2022-05-22 09:50:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-22 10:02:19
 * @FilePath: /algorithm/464_can_i_win/can_i_win.cpp
 */

#include <cassert>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  unordered_map<int, bool> memo;

  bool canIWin(int maxChoosableInteger, int desiredTotal) {
    if ((1 + maxChoosableInteger) * (maxChoosableInteger) / 2 < desiredTotal) {
      return false;
    }
    return dfs(maxChoosableInteger, 0, desiredTotal, 0);
  }

  bool dfs(int maxChoosableInteger, int usedNumbers, int desiredTotal, int currentTotal) {
    if (!memo.count(usedNumbers)) {
      bool res = false;
      for (int i = 0; i < maxChoosableInteger; i++) {
        if (((usedNumbers >> i) & 1) == 0) {
          if (i + 1 + currentTotal >= desiredTotal) {
            res = true;
            break;
          }
          if (!dfs(maxChoosableInteger, usedNumbers | (1 << i), desiredTotal, currentTotal + i + 1)) {
            res = true;
            break;
          }
        }
      }
      memo[usedNumbers] = res;
    }
    return memo[usedNumbers];
  }
};

int main() {
  assert(Solution().canIWin(10, 11) == false);
  assert(Solution().canIWin(10, 0) == true);
  assert(Solution().canIWin(10, 1) == true);
}