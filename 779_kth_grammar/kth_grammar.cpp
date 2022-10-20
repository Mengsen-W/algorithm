/*
 * @Date: 2022-10-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-20
 * @FilePath: /algorithm/779_kth_grammar/kth_grammar.cpp
 */

#include <cassert>

class Solution {
 public:
  int kthGrammar(int n, int k) {
    // return __builtin_popcount(k - 1) & 1;
    k--;
    int res = 0;
    while (k > 0) {
      k &= k - 1;
      res ^= 1;
    }
    return res;
  }
};

int main() {
  assert(Solution().kthGrammar(1, 1) == 0);
  assert(Solution().kthGrammar(2, 1) == 0);
  assert(Solution().kthGrammar(2, 2) == 1);
}