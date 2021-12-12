/*
 * @Date: 2021-12-12 05:21:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-12 05:25:22
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string toLowerCase(string s) {
    for (char& ch : s) {
      if (ch >= 65 && ch <= 90) {
        ch |= 32;
      }
    }
    return s;
  }
};

int main() {
  assert(Solution().toLowerCase("Hello") == "hello");
  assert(Solution().toLowerCase("here") == "here");
  assert(Solution().toLowerCase("LOVELY") == "lovely");
}