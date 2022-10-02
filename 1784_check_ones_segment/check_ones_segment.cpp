/*
 * @Date: 2022-10-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-03
 * @FilePath: /algorithm/1784_check_ones_segment/check_ones_segment.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool checkOnesSegment(string s) { return s.find("01") == string::npos; }
};

int main() {
  assert(!Solution().checkOnesSegment("1001"));
  assert(Solution().checkOnesSegment("110"));
}