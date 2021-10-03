/*
 * @Date: 2021-10-03 08:48:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-03 08:58:10
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  string fractionToDecimal(int numerator, int denominator) {
    if ((int64_t)numerator % denominator == 0)
      return to_string((int64_t)numerator / denominator);

    int64_t up = abs((int64_t)numerator), down = abs((int64_t)denominator);
    string ans(((numerator < 0) ^ (denominator < 0) ? "-" : "") +
               to_string(up / down) + '.');
    unordered_map<int64_t, int> index;

    for (int i = ans.size(); up = up % down * 10; ++i) {
      if (index.count(up)) {
        ans.insert(begin(ans) + index[up], '(');
        ans.push_back(')');
        break;
      }
      index[up] = i;
      ans.push_back('0' + up / down);
    }
    return ans;
  }
};

int main() {
  assert(Solution().fractionToDecimal(1, 2) == "0.5");
  assert(Solution().fractionToDecimal(2, 1) == "2");
  assert(Solution().fractionToDecimal(2, 3) == "0.(6)");
  assert(Solution().fractionToDecimal(4, 333) == "0.(012)");
  assert(Solution().fractionToDecimal(1, 5) == "0.2");
}