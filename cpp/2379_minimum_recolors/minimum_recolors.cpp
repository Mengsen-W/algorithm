/*
 * @Date: 2023-03-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-09
 * @FilePath: /algorithm/cpp/2379_minimum_recolors/minimum_recolors.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int minimumRecolors(string blocks, int k) {
    int l = 0, r = 0, cnt = 0;
    while (r < k) {
      cnt += blocks[r] == 'W' ? 1 : 0;
      r++;
    }
    int res = cnt;
    while (r < blocks.size()) {
      cnt += blocks[r] == 'W' ? 1 : 0;
      cnt -= blocks[l] == 'W' ? 1 : 0;
      res = min(res, cnt);
      l++;
      r++;
    }
    return res;
  }
};

int main() {
  {
    string blocks = "WBBWWBBWBW";
    int k = 7;
    int ans = 3;
    assert(Solution().minimumRecolors(blocks, k) == ans);
  }

  {
    string blocks = "WBWBBBW";
    int k = 2;
    int ans = 0;
    assert(Solution().minimumRecolors(blocks, k) == ans);
  }
}