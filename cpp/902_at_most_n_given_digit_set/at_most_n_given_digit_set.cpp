/*
 * @Date: 2022-10-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-18
 * @FilePath: /algorithm/902_at_most_n_given_digit_set/at_most_n_given_digit_set.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int atMostNGivenDigitSet(vector<string>& digits, int n) {
    string s = to_string(n);
    int m = digits.size(), k = s.size();
    vector<int> bits;
    bool isLimit = true;
    for (int i = 0; i < k; i++) {
      if (!isLimit) {
        bits.emplace_back(m - 1);
      } else {
        int selectIndex = -1;
        for (int j = 0; j < m; j++) {
          if (digits[j][0] <= s[i]) {
            selectIndex = j;
          } else {
            break;
          }
        }
        if (selectIndex >= 0) {
          bits.emplace_back(selectIndex);
          if (digits[selectIndex][0] < s[i]) {
            isLimit = false;
          }
        } else {
          int len = bits.size();
          while (!bits.empty() && bits.back() == 0) {
            bits.pop_back();
          }
          if (!bits.empty()) {
            bits.back() -= 1;
          } else {
            len--;
          }
          while ((int)bits.size() <= len) {
            bits.emplace_back(m - 1);
          }
          isLimit = false;
        }
      }
    }
    int ans = 0;
    for (int i = 0; i < bits.size(); i++) {
      ans = ans * m + (bits[i] + 1);
    }
    return ans;
  }
};

int main() {
  {
    vector<string> digits{"1", "3", "5", "7"};
    int n = 100;
    int ans = 20;
    assert(Solution().atMostNGivenDigitSet(digits, n) == ans);
  }

  {
    vector<string> digits{"1", "4", "9"};
    int n = 1000000000;
    int ans = 29523;
    assert(Solution().atMostNGivenDigitSet(digits, n) == ans);
  }

  {
    vector<string> digits{"7"};
    int n = 8;
    int ans = 1;
    assert(Solution().atMostNGivenDigitSet(digits, n) == ans);
  }
}