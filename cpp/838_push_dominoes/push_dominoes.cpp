/*
 * @Date: 2022-02-21 00:58:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-21 01:04:56
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string pushDominoes(string dominoes) {
    int n = dominoes.size(), i = 0;
    char left = 'L';
    while (i < n) {
      int j = i;
      while (j < n && dominoes[j] == '.') {
        // 找到一段连续的没有被推动的骨牌
        j++;
      }
      char right = j < n ? dominoes[j] : 'R';
      if (left == right) {
        // 方向相同，那么这些竖立骨牌也会倒向同一方向
        while (i < j) {
          dominoes[i++] = right;
        }
      } else if (left == 'R' && right == 'L') {
        // 方向相对，那么就从两侧向中间倒
        int k = j - 1;
        while (i < k) {
          dominoes[i++] = 'R';
          dominoes[k--] = 'L';
        }
      }
      left = right;
      i = j + 1;
    }
    return dominoes;
  }
};

int main() {
  assert(Solution().pushDominoes("RR.L") == "RR.L");
  assert(Solution().pushDominoes(".L.R...LR..L..") == "LL.RR.LLRRLL..");
}