/*
 * @Date: 2023-02-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-12
 * @FilePath: /algorithm/cpp/1138_alphabet_board_path/alphabet_board_path.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string alphabetBoardPath(string target) {
    int cx = 0, cy = 0;
    string res;
    for (char c : target) {
      int nx = (c - 'a') / 5;
      int ny = (c - 'a') % 5;
      if (nx < cx) {
        res.append(cx - nx, 'U');
      }
      if (ny < cy) {
        res.append(cy - ny, 'L');
      }
      if (nx > cx) {
        res.append(nx - cx, 'D');
      }
      if (ny > cy) {
        res.append(ny - cy, 'R');
      }
      res.push_back('!');
      cx = nx;
      cy = ny;
    }
    return res;
  }
};

int main() {
  {
    string target = "leet";
    string ans = "DDR!UURRR!!DDD!";
    assert(Solution().alphabetBoardPath(target) == ans);
  }

  {
    string target = "code";
    string ans = "RR!DDRR!UUL!R!";
    assert(Solution().alphabetBoardPath(target) == ans);
  }
}