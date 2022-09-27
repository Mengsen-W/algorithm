/*
 * @Date: 2022-09-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-27
 * @FilePath: /algorithm/01.02_check_permutation/check_permutation.cpp
 */

#include <assert.h>

#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  bool CheckPermutation(string s1, string s2) {
    if (s1.length() != s2.length()) {
      return false;
    }
    vector<int> table(26, 0);
    for (auto& ch : s1) {
      table[ch - 'a']++;
    }
    for (auto& ch : s2) {
      table[ch - 'a']--;
      if (table[ch - 'a'] < 0) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  assert(Solution().CheckPermutation("abc", "bca"));
  assert(!Solution().CheckPermutation("abc", "bad"));
}