/*
 * @Date: 2023-04-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-06
 * @FilePath: /algorithm/cpp/1017_base_neg2/base_neg2.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string baseNeg2(int n) {
    int val = 0x55555555 ^ (0x55555555 - n);
    if (val == 0) {
      return "0";
    }
    string res;
    while (val) {
      res.push_back('0' + (val & 1));
      val >>= 1;
    }
    reverse(res.begin(), res.end());
    return res;
  }
};

int main() {
  {
    int n = 2;
    string ans = "110";
    assert(Solution().baseNeg2(n) == ans);
  }

  {
    int n = 3;
    string ans = "111";
    assert(Solution().baseNeg2(n) == ans);
  }

  {
    int n = 4;
    string ans = "100";
    assert(Solution().baseNeg2(n) == ans);
  }
}
