/*
 * @Date: 2022-12-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-27
 * @FilePath: /algorithm/2027_minimum_moves/minimum_moves.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int minimumMoves(string s) {
    int covered = -1, res = 0;
    for (int i = 0; i < s.size(); i++) {
      if (s[i] == 'X' && i > covered) {
        res += 1;
        covered = i + 2;
      }
    }
    return res;
  }
};

int main() {
  {
    string s{"XXX"};
    int ans = 1;
    assert(Solution().minimumMoves(s) == ans);
  }

  {
    string s{"XXOX"};
    int ans = 2;
    assert(Solution().minimumMoves(s) == ans);
  }

  {
    string s{"OOOO"};
    int ans = 0;
    assert(Solution().minimumMoves(s) == ans);
  }
}