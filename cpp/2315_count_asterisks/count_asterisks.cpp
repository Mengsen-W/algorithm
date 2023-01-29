/*
 * @Date: 2023-01-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-29
 * @FilePath: /algorithm/cpp/2315_count_asterisks/count_asterisks.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int countAsterisks(string s) {
    bool valid = true;
    int res = 0;
    for (int i = 0; i < s.size(); i++) {
      char c = s[i];
      if (c == '|') {
        valid = !valid;
      } else if (c == '*' && valid) {
        res++;
      }
    }
    return res;
  }
};

int main() {
  {
    string s{"l|*e*et|c**o|*de|"};
    int ans = 2;
    assert(Solution().countAsterisks(s) == ans);
  }

  {
    string s{"iamprogrammer"};
    int ans = 0;
    assert(Solution().countAsterisks(s) == ans);
  }

  {
    string s{"yo|uar|e**|b|e***au|tifu|l"};
    int ans = 5;
    assert(Solution().countAsterisks(s) == ans);
  }
}