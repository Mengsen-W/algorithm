/*
 * @Date: 2022-12-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-03
 * @FilePath: /algorithm/1796_second_highest/second_highest.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int secondHighest(string s) {
    int first = -1, second = -1;
    for (auto c : s) {
      if (isdigit(c)) {
        int num = c - '0';
        if (num > first) {
          second = first;
          first = num;
        } else if (num < first && num > second) {
          second = num;
        }
      }
    }
    return second;
  }
};

int main() {
  assert(Solution().secondHighest("dfa12321afd") == 2);
  assert(Solution().secondHighest("abc1111") == -1);
}