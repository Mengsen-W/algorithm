/*
 * @Date: 2023-04-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-30
 * @FilePath: /algorithm/cpp/1033_num_moves_stones/num_moves_stones.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> numMovesStones(int a, int b, int c) {
    int x = min({a, b, c});
    int z = max({a, b, c});
    int y = a + b + c - x - z;

    vector<int> res(2);
    res[0] = 2;
    if ((z - y) == 1 && (y - x) == 1) {
      res[0] = 0;
    } else if ((z - y) <= 2 || (y - x) <= 2) {
      res[0] = 1;
    }
    res[1] = (z - x - 2);
    return res;
  }
};

int main() {
  {
    int a = 1, b = 2, c = 5;
    vector<int> ans{1, 2};
    assert(Solution().numMovesStones(a, b, c) == ans);
  }

  {
    int a = 4, b = 3, c = 2;
    vector<int> ans{0, 0};
    assert(Solution().numMovesStones(a, b, c) == ans);
  }
}