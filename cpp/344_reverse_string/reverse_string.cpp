/*
 * @Date: 2023-08-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-07
 * @FilePath: /algorithm/cpp/344_reverse_string/reverse_string.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  void reverseString(vector<char>& s) {
    int n = s.size();
    for (int left = 0, right = n - 1; left < right; ++left, --right) {
      swap(s[left], s[right]);
    }
  }
};

int main() {
  Solution s;
  vector<tuple<vector<char>, vector<char>>> tests{
      {{'h', 'e', 'l', 'l', 'o'}, {'o', 'l', 'l', 'e', 'h'}},
      {{'H', 'a', 'n', 'n', 'a', 'h'}, {'h', 'a', 'n', 'n', 'a', 'H'}},
  };

  for (auto& [input, output] : tests) {
    s.reverseString(input);
    assert(input == output);
  }
}
