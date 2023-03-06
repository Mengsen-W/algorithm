/*
 * @Date: 2023-03-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-06
 * @FilePath: /algorithm/cpp/1653_minimum_deletions/minimum_deletions.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int minimumDeletions(string s) {
    int leftb = 0, righta = 0;
    for (int i = 0; i < s.size(); i++) {
      if (s[i] == 'a') {
        righta++;
      }
    }
    int res = righta;
    for (int i = 0; i < s.size(); i++) {
      char c = s[i];
      if (c == 'a') {
        righta--;
      } else {
        leftb++;
      }
      res = min(res, leftb + righta);
    }
    return res;
  }
};

int main() {
  {
    string s{"aababbab"};
    int ans = 2;
    assert(Solution().minimumDeletions(s) == ans);
  }

  {
    string s{"bbaaaaabb"};
    int ans = 2;
    assert(Solution().minimumDeletions(s) == ans);
  }
}
