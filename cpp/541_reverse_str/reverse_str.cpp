/*
 * @Date: 2021-08-20 15:16:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-21 14:01:11
 */

#include <algorithm>
#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string reverseStr(string s, int k) {
    int n = s.length();
    for (int i = 0; i < n; i += 2 * k) {
      reverse(s.begin() + i, s.begin() + min(i + k, n));
    }
    return s;
  }
};

int main() {
  {
    string s = "abcdefg";
    int k = 2;
    string ans = "bacdfeg";
    assert(Solution{}.reverseStr(s, k) == ans);
  }
  {
    string s = "abcd";
    int k = 2;
    string ans = "bacd";
    assert(Solution{}.reverseStr(s, k) == ans);
  }
}