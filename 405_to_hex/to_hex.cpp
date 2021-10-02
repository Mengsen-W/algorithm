/*
 * @Date: 2021-10-02 08:34:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-02 08:39:01
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string toHex(int num) {
    if (num == 0) return "0";

    string sb;
    for (int i = 7; i >= 0; i--) {
      int val = (num >> (4 * i)) & 0xf;
      if (sb.length() > 0 || val > 0) {
        char digit = val < 10 ? (char)('0' + val) : (char)('a' + val - 10);
        sb.push_back(digit);
      }
    }
    return sb;
  }
};

int main() {
  assert(Solution().toHex(26) == "1a");
  assert(Solution().toHex(-1) == "ffffffff");
}