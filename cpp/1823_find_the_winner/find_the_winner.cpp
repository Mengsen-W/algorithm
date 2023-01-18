/*
 * @Date: 2022-05-04 08:11:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-04 08:18:46
 * @FilePath: /algorithm/1823_find_the_winner/find_the_winner.cpp
 */

#include <cassert>

class Solution {
 public:
  int findTheWinner(int n, int k) {
    int winner = 1;
    for (int i = 2; i <= n; i++) {
      winner = (k + winner - 1) % i + 1;
    }
    return winner;
  }
};

int main() {
  assert(Solution().findTheWinner(5, 2) == 3);
  assert(Solution().findTheWinner(6, 5) == 1);
}
