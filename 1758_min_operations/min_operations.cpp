/*
 * @Date: 2022-11-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-29
 * @FilePath: /algorithm/1758_min_operations/min_operations.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int minOperations(string s) {
    int cnt = 0;
    for (int i = 0; i < s.size(); i++) {
      char c = s[i];
      if (c != ('0' + i % 2)) {
        cnt++;
      }
    }
    return min(cnt, (int)s.size() - cnt);
  }
};

int main() {
  assert(Solution().minOperations("0100") == 1);
  assert(Solution().minOperations("10") == 0);
  assert(Solution().minOperations("1111") == 2);
}
