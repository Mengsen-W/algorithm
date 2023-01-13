/*
 * @Date: 2023-01-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-13
 * @FilePath: /algorithm/2287_rearrange_characters/rearrange_characters.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  int rearrangeCharacters(string s, string target) {
    unordered_map<char, int> sCounts, targetCounts;
    int n = s.size(), m = target.size();
    for (int i = 0; i < m; i++) {
      targetCounts[target[i]]++;
    }
    for (int i = 0; i < n; i++) {
      if (targetCounts.count(s[i])) {
        sCounts[s[i]]++;
      }
    }
    int ans = INT_MAX;
    for (auto &[c, count] : targetCounts) {
      int totalCount = sCounts[c];
      ans = min(ans, totalCount / count);
      if (ans == 0) {
        return 0;
      }
    }
    return ans;
  }
};

int main() {
  {
    string s{"ilovecodingonleetcode"};
    string target{"code"};
    int ans = 2;
    assert(Solution().rearrangeCharacters(s, target) == ans);
  }

  {
    string s{"abcba"};
    string target{"abc"};
    int ans = 1;
    assert(Solution().rearrangeCharacters(s, target) == ans);
  }

  {
    string s{"abbaccaddaeea"};
    string target{"aaaaa"};
    int ans = 1;
    assert(Solution().rearrangeCharacters(s, target) == ans);
  }
}