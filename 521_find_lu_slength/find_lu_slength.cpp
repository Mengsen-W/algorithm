/*
 * @Date: 2022-03-05 00:33:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-05 00:46:54
 * @FilePath: /algorithm/521_find_lu_slength/find_lu_slength.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int findLUSlength(string a, string b) {
    return a == b ? -1 : max(a.length(), b.length());
  }
};

int main() {
  assert(Solution().findLUSlength("aba", "cdc") == 3);
  assert(Solution().findLUSlength("aaa", "bbb") == 3);
  assert(Solution().findLUSlength("aaa", "aaa") == -1);
}