/*
 * @Date: 2023-05-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-03
 * @FilePath: /algorithm/cpp/1003_is_valid/is_valid.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool isValid(string s) {
    string stk;
    for (auto c : s) {
      stk.push_back(c);
      if (stk.size() >= 3 && stk.substr(stk.size() - 3, 3) == "abc") {
        stk.erase(stk.end() - 3, stk.end());
      }
    }
    return stk.empty();
  }
};

int main() {
  {
    string s = "aabcbc";
    bool ans = true;
    assert(Solution().isValid(s) == ans);
  }

  {
    string s = "abcabcababcc";
    bool ans = true;
    assert(Solution().isValid(s) == ans);
  }

  {
    string s = "abccba";
    bool ans = false;
    assert(Solution().isValid(s) == ans);
  }
}