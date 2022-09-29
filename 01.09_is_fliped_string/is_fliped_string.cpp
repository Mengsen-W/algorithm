/*
 * @Date: 2022-09-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-29
 * @FilePath: /algorithm/01.09_is_fliped_string/is_fliped_string.cpp
 */

#include <assert.h>

#include <string>

using namespace std;

class Solution {
 public:
  bool isFlipedString(string s1, string s2) { return s1.size() == s2.size() && (s1 + s1).find(s2) != string::npos; }
};

int main() {
  assert(Solution().isFlipedString("waterbottle", "erbottlewat"));
  assert(!Solution().isFlipedString("aa", "aba"));
}
