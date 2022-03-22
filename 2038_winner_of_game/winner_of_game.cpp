/*
 * @Date: 2022-03-22 00:44:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-22 00:48:29
 * @FilePath: /algorithm/2038_winner_of_game/winner_of_game.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool winnerOfGame(string colors) {
    int freq[2] = {0, 0};
    char cur = 'C';
    int cnt = 0;
    for (char c : colors) {
      if (c != cur) {
        cur = c;
        cnt = 1;
      } else if (++cnt >= 3) {
        ++freq[cur - 'A'];
      }
    }
    return freq[0] > freq[1];
  }
};

int main() {
  assert(Solution().winnerOfGame("AAABABB") == true);
  assert(Solution().winnerOfGame("AA") == false);
  assert(Solution().winnerOfGame("ABBBBBBBAAA") == false);
}