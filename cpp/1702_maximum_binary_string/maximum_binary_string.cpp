/*
 * @Date: 2024-04-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-10
 * @FilePath: /algorithm/cpp/1702_maximum_binary_string/maximum_binary_string.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string maximumBinaryString(string binary) {
    int n = binary.size(), i = binary.find('0');
    if (i == string::npos) {
      return binary;
    }
    int zeros = count(binary.begin(), binary.end(), '0');
    string s(n, '1');
    s[i + zeros - 1] = '0';
    return s;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"000110", "111011"},
      {"01", "01"},
  };

  for (auto &[binary, ans] : tests) {
    assert(Solution().maximumBinaryString(binary) == ans);
  }
}