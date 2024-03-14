/*
 * @Date: 2024-03-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-13
 * @FilePath: /algorithm/cpp/2864_maximum_odd_binary_number/maximum_odd_binary_number.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string maximumOddBinaryNumber(string s) {
    int cnt = count(s.begin(), s.end(), '1');
    return string(cnt - 1, '1') + string(s.length() - cnt, '0') + '1';
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"010", "001"},
      {"0101", "1001"},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().maximumOddBinaryNumber(s) == ans);
  }
}