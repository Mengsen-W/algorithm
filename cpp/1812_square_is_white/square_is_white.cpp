/*
 * @Date: 2022-12-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-08
 * @FilePath: /algorithm/1812_square_is_white/square_is_white.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool squareIsWhite(string coordinates) { return ((coordinates[0] - 'a' + 1) + (coordinates[1] - '0')) % 2 == 1; }
};

int main() {
  assert(!Solution().squareIsWhite("a1"));
  assert(Solution().squareIsWhite("h3"));
  assert(!Solution().squareIsWhite("c7"));
}