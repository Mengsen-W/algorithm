/*
 * @Date: 2022-10-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-11
 * @FilePath: /algorithm/1790_are_almost_equal/are_almost_equal.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  bool areAlmostEqual(string s1, string s2) {
    int n = s1.size();
    vector<int> diff;
    for (int i = 0; i < n; ++i) {
      if (s1[i] != s2[i]) {
        if (diff.size() >= 2) {
          return false;
        }
        diff.emplace_back(i);
      }
    }
    if (diff.size() == 0) {
      return true;
    }
    if (diff.size() != 2) {
      return false;
    }
    return s1[diff[0]] == s2[diff[1]] && s1[diff[1]] == s2[diff[0]];
  }
};

int main() {
  assert(Solution().areAlmostEqual("bank", "kanb"));
  assert(!Solution().areAlmostEqual("attack", "defend"));
  assert(Solution().areAlmostEqual("kelb", "kelb"));
  assert(!Solution().areAlmostEqual("abcd", "dcba"));
}