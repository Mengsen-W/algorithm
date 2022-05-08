/*
 * @Date: 2022-05-09 07:38:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-09 07:40:45
 * @FilePath: /algorithm/942_di_string_match/di_string_match.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> diStringMatch(string s) {
    int n = s.length(), lo = 0, hi = n;
    vector<int> perm(n + 1);
    for (int i = 0; i < n; ++i) {
      perm[i] = s[i] == 'I' ? lo++ : hi--;
    }
    perm[n] = lo;  // 最后剩下一个数，此时 lo == hi
    return perm;
  }
};

int main() {
  assert((Solution().diStringMatch("IDID") == vector<int>{0, 4, 1, 3, 2}));
  assert((Solution().diStringMatch("III") == vector<int>{0, 1, 2, 3}));
  assert((Solution().diStringMatch("DDI") == vector<int>{3, 2, 0, 1}));
}
