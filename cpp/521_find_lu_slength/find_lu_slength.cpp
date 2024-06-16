/*
 * @Date: 2022-03-05 00:33:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-05 00:46:54
 * @FilePath: /algorithm/521_find_lu_slength/find_lu_slength.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findLUSlength(string a, string b) { return a == b ? -1 : max(a.length(), b.length()); }
};

int main() {
  vector<tuple<string, string, int>> tests{
      {"aba", "cdc", 3},
      {"aaa", "bbb", 3},
      {"aaa", "aaa", -1},
  };

  for (auto &[a, b, ans] : tests) {
    assert(Solution().findLUSlength(a, b) == ans);
  }
}