/*
 * @Date: 2022-04-19 07:19:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-19 07:24:22
 * @FilePath: /algorithm/821_shortest_to_char/shortest_to_char.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> shortestToChar(string s, char c) {
    int n = s.length();
    vector<int> ans(n);

    for (int i = 0, idx = -n; i < n; ++i) {
      if (s[i] == c) {
        idx = i;
      }
      ans[i] = i - idx;
    }

    for (int i = n - 1, idx = 2 * n; i >= 0; --i) {
      if (s[i] == c) {
        idx = i;
      }
      ans[i] = min(ans[i], idx - i);
    }
    return ans;
  }
};

int main() {
  assert(Solution().shortestToChar("loveleetcode", 'e') == vector<int>({3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0}));
  assert(Solution().shortestToChar("aaab", 'b') == vector<int>({3, 2, 1, 0}));
}