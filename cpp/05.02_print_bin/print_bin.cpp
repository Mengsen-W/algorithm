/*
 * @Date: 2023-03-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-02
 * @FilePath: /algorithm/cpp/05.02_print_bin/print_bin.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string printBin(double num) {
    string res = "0.";
    while (res.size() <= 32 && num != 0) {
      num *= 2;
      int digit = num;
      res.push_back(digit + '0');
      num -= digit;
    }
    return res.size() <= 32 ? res : "ERROR";
  }
};

int main() {
  {
    double num = 0.625;
    string ans = "0.101";
    assert(Solution().printBin(num) == ans);
  }

  {
    double num = 0.1;
    string ans = "ERROR";
    assert(Solution().printBin(num) == ans);
  }
}
