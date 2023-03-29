/*
 * @Date: 2023-03-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-29
 * @FilePath: /algorithm/cpp/1641_count_vowel_strings/count_vowel_strings.cpp
 */

#include <cassert>

class Solution {
 public:
  int countVowelStrings(int n) { return (n + 1) * (n + 2) * (n + 3) * (n + 4) / 24; }
};

int main() {
  assert(Solution().countVowelStrings(1) == 5);
  assert(Solution().countVowelStrings(2) == 15);
}