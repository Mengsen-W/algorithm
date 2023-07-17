/*
 * @Date: 2023-07-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-17
 * @FilePath: /algorithm/cpp/415_add_strings/add_strings.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string addStrings(string num1, string num2) {
    int i = num1.length() - 1, j = num2.length() - 1, add = 0;
    string ans = "";
    while (i >= 0 || j >= 0 || add != 0) {
      int x = i >= 0 ? num1[i] - '0' : 0;
      int y = j >= 0 ? num2[j] - '0' : 0;
      int result = x + y + add;
      ans.push_back('0' + result % 10);
      add = result / 10;
      i -= 1;
      j -= 1;
    }
    // 计算完以后的答案需要翻转过来
    reverse(ans.begin(), ans.end());
    return ans;
  }
};

int main() {
  vector<tuple<string, string, string>> tests{
      {"11", "123", "134"},
      {"456", "77", "533"},
      {"0", "0", "0"},
  };

  for (auto &[num1, num2, ans] : tests) {
    assert(Solution().addStrings(num1, num2) == ans);
  }
}