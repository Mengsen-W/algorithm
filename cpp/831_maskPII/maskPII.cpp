/*
 * @Date: 2023-04-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-01
 * @FilePath: /algorithm/cpp/831_maskPII/maskPII.cpp
 */

#include <cassert>
#include <regex>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> country = {"", "+*-", "+**-", "+***-"};

  string maskPII(string s) {
    string res;
    int at = s.find("@");
    if (at != string::npos) {
      transform(s.begin(), s.end(), s.begin(), ::tolower);
      return s.substr(0, 1) + "*****" + s.substr(at - 1);
    }
    s = regex_replace(s, regex("[^0-9]"), "");
    return country[s.size() - 10] + "***-***-" + s.substr(s.size() - 4);
  }
};

int main() {
  {
    string s = "LeetCode@LeetCode.com";
    string ans = "l*****e@leetcode.com";
    assert(Solution().maskPII(s) == ans);
  }

  {
    string s = "AB@qq.com";
    string ans = "a*****b@qq.com";
    assert(Solution().maskPII(s) == ans);
  }

  {
    string s = "1(234)567-890";
    string ans = "***-***-7890";
    assert(Solution().maskPII(s) == ans);
  }

  {
    string s = "86-(10)12345678";
    string ans = "+**-***-***-5678";
    assert(Solution().maskPII(s) == ans);
  }
}
