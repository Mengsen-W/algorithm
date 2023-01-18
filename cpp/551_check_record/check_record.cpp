/*
 * @Date: 2021-08-17 09:24:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-17 09:31:22
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool checkRecord(string s) {
    int absents = 0, lates = 0;
    for (auto &ch : s) {
      if (ch == 'A') {
        absents++;
        if (absents >= 2) {
          return false;
        }
      }
      if (ch == 'L') {
        lates++;
        if (lates >= 3) {
          return false;
        }
      } else {
        lates = 0;
      }
    }
    return true;
  }
};

int main() {
  {
    string s = "PPALLP";
    assert(Solution().checkRecord(s));
  }
  {
    string s = "PPALLL";
    assert(!Solution().checkRecord(s));
  }
}