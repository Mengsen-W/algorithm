/*
 * @Date: 2023-01-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2023-01-03
 * @FilePath: /algorithm/2024_are_numbers_ascending/are_numbers_ascending.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool areNumbersAscending(string s) {
    int pre = 0, pos = 0;
    while (pos < s.size()) {
      if (isdigit(s[pos])) {
        int cur = 0;
        while (pos < s.size() && isdigit(s[pos])) {
          cur = cur * 10 + s[pos] - '0';
          pos++;
        }
        if (cur <= pre) {
          return false;
        }
        pre = cur;
      } else {
        pos++;
      }
    }
    return true;
  }
};

int main() {
  {
    string s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles";
    bool ans = true;
    assert(Solution().areNumbersAscending(s) == ans);
  }

  {
    string s = "hello world 5 x 5";
    bool ans = false;
    assert(Solution().areNumbersAscending(s) == ans);
  }

  {
    string s = "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s";
    bool ans = false;
    assert(Solution().areNumbersAscending(s) == ans);
  }

  {
    string s = "4 5 11 26";
    bool ans = true;
    assert(Solution().areNumbersAscending(s) == ans);
  }
}