/*
 * @Date: 2022-10-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-04
 * @FilePath: /algorithm/921_min_add_to_make_valid/min_add_to_make_valid.cpp
 */

#include <cassert>
#include <string>

class Solution {
 public:
  int minAddToMakeValid(std::string s) {
    // ans total right, cnt total remind left
    int ans = 0, cnt = 0;
    for (auto &c : s) {
      if (c == '(') {
        cnt++;
      } else {
        if (cnt > 0) {
          cnt--;
        } else {
          ans++;
        }
      }
    }
    return ans + cnt;
  }
};

int main() {
  assert(Solution().minAddToMakeValid("())") == 1);
  assert(Solution().minAddToMakeValid("(((") == 3);
}