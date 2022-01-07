/*
 * @Date: 2022-01-07 01:08:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-07 01:20:11
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int maxDepth(string s) {
    int ans = 0, size = 0;
    for (char ch : s) {
      if (ch == '(') {
        ++size;
        ans = max(ans, size);
      } else if (ch == ')') {
        --size;
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().maxDepth("(1+(2*3)+((8)/4))+1") == 3);
  assert(Solution().maxDepth("(1)+((2))+(((3)))") == 3);
  assert(Solution().maxDepth("1+(2*3)/(2-1)") == 1);
  assert(Solution().maxDepth("1") == 0);
}