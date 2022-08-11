/*
 * @Date: 2022-08-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-11
 * @FilePath: /algorithm/1417_reformat/reformat.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string reformat(string s) {
    int sum_digit = 0;
    for (auto& c : s) {
      if (isdigit(c)) {
        sum_digit++;
      }
    }
    int sum_alpha = s.size() - sum_digit;
    if (abs(sum_digit - sum_alpha) > 1) {
      return "";
    }
    bool flag = sum_digit > sum_alpha;
    for (int i = 0, j = 1; i < s.size(); i += 2) {
      if (isdigit(s[i]) != flag) {
        while (isdigit(s[j]) != flag) {
          j += 2;
        }
        swap(s[i], s[j]);
      }
    }
    return s;
  }
};

int main() {
  assert(Solution().reformat("a0b1c2") == "0a1b2c");
  assert(Solution().reformat("leetcode") == "");
  assert(Solution().reformat("1229857369") == "");
  assert(Solution().reformat("covid2019") == "c2o0v1i9d");
  assert(Solution().reformat("ab123") == "1a2b3");
}