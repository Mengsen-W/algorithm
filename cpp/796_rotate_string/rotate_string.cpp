/*
 * @Date: 2022-04-07 01:43:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-07 01:48:23
 * @FilePath: /algorithm/796_rotate_string/rotate_string.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool rotateString(string s, string goal) {
    return s.size() == goal.size() && (s + s).find(goal) != string::npos;
  }
};

int main() {
  assert(Solution().rotateString("abcde", "cdeab") == true);
  assert(Solution().rotateString("abcde", "abced") == false);
}