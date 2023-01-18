/*
 * @Date: 2021-09-07 16:56:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-07 17:02:46
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int balancedStringSplit(string s) {
    int ans = 0, d = 0;
    for (char ch : s) {
      ch == 'L' ? ++d : --d;
      if (d == 0) ++ans;
    }
    return ans;
  }
};

int main() {
  {
    string s = "RLRRLLRLRL";
    int ans = 4;
    assert(Solution().balancedStringSplit(s) == ans);
  }
  {
    string s = "RLLLLRRRLR";
    int ans = 3;
    assert(Solution().balancedStringSplit(s) == ans);
  }
  {
    string s = "LLLLRRRR";
    int ans = 1;
    assert(Solution().balancedStringSplit(s) == ans);
  }
  {
    string s = "RLRRRLLRLL";
    int ans = 2;
    assert(Solution().balancedStringSplit(s) == ans);
  }
}