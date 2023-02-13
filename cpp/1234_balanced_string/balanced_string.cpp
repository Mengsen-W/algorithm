/*
 * @Date: 2023-02-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-13
 * @FilePath: /algorithm/cpp/1234_balanced_string/balanced_string.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int idx(const char& c) { return c - 'A'; }

  int balancedString(string s) {
    vector<int> cnt(26);
    for (auto c : s) {
      cnt[idx(c)]++;
    }

    int partial = s.size() / 4;
    int res = s.size();
    auto check = [&]() {
      if (cnt[idx('Q')] > partial || cnt[idx('W')] > partial || cnt[idx('E')] > partial || cnt[idx('R')] > partial) {
        return false;
      }
      return true;
    };

    if (check()) {
      return 0;
    }
    for (int l = 0, r = 0; l < s.size(); l++) {
      while (r < s.size() && !check()) {
        cnt[idx(s[r])]--;
        r++;
      }
      if (!check()) {
        break;
      }
      res = min(res, r - l);
      cnt[idx(s[l])]++;
    }
    return res;
  }
};

int main() {
  {
    string s{"QWER"};
    int ans = 0;
    assert(Solution().balancedString(s) == ans);
  }

  {
    string s{"QQWE"};
    int ans = 1;
    assert(Solution().balancedString(s) == ans);
  }

  {
    string s{"QQQW"};
    int ans = 2;
    assert(Solution().balancedString(s) == ans);
  }

  {
    string s{"QQQQ"};
    int ans = 3;
    assert(Solution().balancedString(s) == ans);
  }
}