/*
 * @Date: 2022-11-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-13
 * @FilePath: /algorithm/791_custom_sort_string/custom_sort_string.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  string customSortString(string order, string s) {
    vector<int> freq(26);
    for (char ch : s) {
      ++freq[ch - 'a'];
    }
    string ans;
    for (char ch : order) {
      if (freq[ch - 'a'] > 0) {
        ans += string(freq[ch - 'a'], ch);
        freq[ch - 'a'] = 0;
      }
    }
    for (int i = 0; i < 26; ++i) {
      if (freq[i] > 0) {
        ans += string(freq[i], i + 'a');
      }
    }
    return ans;
  }
};

int main() {
  {
    string order = "cba";
    string s = "abcd";
    string ans = "cbad";
    assert(Solution().customSortString(order, s) == ans);
  }

  {
    string order = "cbafg";
    string s = "abcd";
    string ans = "cbad";
    assert(Solution().customSortString(order, s) == ans);
  }
}