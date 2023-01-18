/*
 * @Date: 2023-01-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2023-01-01
 * @FilePath: /algorithm/2351_repeated_character/repeated_character.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  char repeatedCharacter(string s) {
    int seen = 0;
    for (char ch : s) {
      int x = ch - 'a';
      if (seen & (1 << x)) {
        return ch;
      }
      seen |= (1 << x);
    }
    // impossible
    return ' ';
  }
};

int main() {
  {
    string s{"abccbaacz"};
    char ans = 'c';
    assert(Solution().repeatedCharacter(s) == ans);
  }

  {
    string s{"abcdd"};
    char ans = 'd';
    assert(Solution().repeatedCharacter(s) == ans);
  }
}
