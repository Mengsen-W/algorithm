/*
 * @Date: 2023-02-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-25
 * @FilePath: /algorithm/cpp/1247_minimum_swap/minimum_swap.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int minimumSwap(string s1, string s2) {
    int xy = 0, yx = 0;
    int n = s1.size();
    for (int i = 0; i < n; i++) {
      char a = s1[i], b = s2[i];
      if (a == 'x' and b == 'y') {
        xy++;
      }
      if (a == 'y' and b == 'x') {
        yx++;
      }
    }
    if ((xy + yx) % 2 == 1) {
      return -1;
    }
    return xy / 2 + yx / 2 + xy % 2 + yx % 2;
  }
};

int main() {
  {
    string s1{"xx"};
    string s2{"yy"};
    int ans = 1;
    assert(Solution().minimumSwap(s1, s2) == ans);
  }

  {
    string s1{"xy"};
    string s2{"yx"};
    int ans = 2;
    assert(Solution().minimumSwap(s1, s2) == ans);
  }

  {
    string s1{"xx"};
    string s2{"xy"};
    int ans = -1;
    assert(Solution().minimumSwap(s1, s2) == ans);
  }

  {
    string s1{"xxyyxyxyxx"};
    string s2{"xyyxyxxxyx"};
    int ans = 4;
    assert(Solution().minimumSwap(s1, s2) == ans);
  }
}