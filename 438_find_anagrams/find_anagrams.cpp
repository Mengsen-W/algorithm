/*
 * @Date: 2021-11-28 02:17:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-28 02:21:40
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findAnagrams(string s, string p) {
    int sLen = s.size(), pLen = p.size();

    if (sLen < pLen) return vector<int>();

    vector<int> ans;
    vector<int> count(26);
    for (int i = 0; i < pLen; ++i) {
      ++count[s[i] - 'a'];
      --count[p[i] - 'a'];
    }

    int differ = 0;
    for (int j = 0; j < 26; ++j)
      if (count[j] != 0) ++differ;

    if (differ == 0) ans.emplace_back(0);

    for (int i = 0; i < sLen - pLen; ++i) {
      if (count[s[i] - 'a'] == 1)
        --differ;
      else if (count[s[i] - 'a'] == 0)
        ++differ;
      --count[s[i] - 'a'];
      if (count[s[i + pLen] - 'a'] == -1)
        --differ;
      else if (count[s[i + pLen] - 'a'] == 0)
        ++differ;
      ++count[s[i + pLen] - 'a'];
      if (differ == 0) ans.emplace_back(i + 1);
    }

    return ans;
  }
};

int main() {
  assert((Solution().findAnagrams("cbaebabacd", "abc") == vector<int>{0, 6}));
  assert((Solution().findAnagrams("abab", "ab") == vector<int>{0, 1, 2}));
}