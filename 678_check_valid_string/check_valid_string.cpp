/*
 * @Date: 2021-09-12 08:19:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-12 08:26:03
 */

#include <cassert>
#include <cmath>
#include <string>

using namespace std;

class Solution {
 public:
  bool checkValidString(string s) {
    int minCount = 0, maxCount = 0;
    int n = s.size();
    for (int i = 0; i < n; i++) {
      char c = s[i];
      if (c == '(') {
        minCount++;
        maxCount++;
      } else if (c == ')') {
        minCount = max(minCount - 1, 0);
        maxCount--;
        if (maxCount < 0) return false;
      } else {
        minCount = max(minCount - 1, 0);
        maxCount++;
      }
    }
    return minCount == 0;
  }
};

int main() {
  {
    string s = "()";
    assert(Solution().checkValidString(s));
  }
  {
    string s = "(*)";
    assert(Solution().checkValidString(s));
  }
  {
    string s = "(*))";
    assert(Solution().checkValidString(s));
  }
  return 0;
}
