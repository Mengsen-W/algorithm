/*
 * @Date: 2022-12-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-15
 * @FilePath: /algorithm/1945_get_lucky/get_lucky.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int getLucky(string s, int k) {
    string digits;
    for (char ch : s) {
      digits += to_string(ch - 'a' + 1);
    }

    for (int i = 1; i <= k && digits.size() > 1; ++i) {
      int sum = 0;
      for (char ch : digits) {
        sum += ch - '0';
      }
      digits = to_string(sum);
    }

    return stoi(digits);
  }
};

int main() {
  {
    string s{"iiii"};
    int k = 1;
    int ans = 36;
    assert(Solution().getLucky(s, k) == ans);
  }

  {
    string s{"leetcode"};
    int k = 2;
    int ans = 6;
    assert(Solution().getLucky(s, k) == ans);
  }
}
