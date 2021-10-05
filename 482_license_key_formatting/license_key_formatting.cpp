/*
 * @Date: 2021-10-04 17:48:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-04 17:54:45
 */

#include <algorithm>
#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string licenseKeyFormatting(string s, int k) {
    string ans;
    int cnt = 0;

    for (int i = s.size() - 1; i >= 0; i--) {
      if (s[i] != '-') {
        ans.push_back(toupper(s[i]));
        cnt++;
        if (cnt % k == 0) ans.push_back('-');
      }
    }
    if (ans.size() > 0 && ans.back() == '-') ans.pop_back();
    reverse(ans.begin(), ans.end());

    return ans;
  }
};

int main() {
  {
    string s = "5F3Z-2e-9-w";
    int k = 4;
    string ans = "5F3Z-2E9W";
    assert(Solution().licenseKeyFormatting(s, k) == ans);
  }
  {
    string s = "2-5g-3-J";
    int k = 2;
    string ans = "2-5G-3J";
    assert(Solution().licenseKeyFormatting(s, k) == ans);
  }
}