/*
 * @Date: 2022-11-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-08
 * @FilePath: /algorithm/1684_count_consistent_strings/count_consistent_strings.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int countConsistentStrings(string allowed, vector<string>& words) {
    int mask = 0;
    for (auto c : allowed) {
      mask |= 1 << (c - 'a');
    }
    int res = 0;
    for (auto&& word : words) {
      int mask1 = 0;
      for (auto c : word) {
        mask1 |= 1 << (c - 'a');
      }
      if ((mask1 | mask) == mask) {
        res++;
      }
    }
    return res;
  }
};

int main() {
  {
    string allowed{"ab"};
    vector<string> words{"ad", "bd", "aaab", "baa", "badab"};
    int ans = 2;
    assert(Solution().countConsistentStrings(allowed, words) == ans);
  }

  {
    string allowed{"abc"};
    vector<string> words{"a", "b", "c", "ab", "ac", "bc", "abc"};
    int ans = 7;
    assert(Solution().countConsistentStrings(allowed, words) == ans);
  }

  {
    string allowed{"cad"};
    vector<string> words{"cc", "acd", "b", "ba", "bac", "bad", "ac", "d"};
    int ans = 4;
    assert(Solution().countConsistentStrings(allowed, words) == ans);
  }
}