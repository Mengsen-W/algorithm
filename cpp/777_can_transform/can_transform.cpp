/*
 * @Date: 2022-10-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-02
 * @FilePath: /algorithm/777_can_transform/can_transform.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool canTransform(string start, string end) {
    int n = start.size();
    int i = 0, j = 0;
    while (i < n && j < n) {
      while (i < n && start[i] == 'X') {
        i++;
      }
      while (j < n && end[j] == 'X') {
        j++;
      }
      if (i < n && j < n) {
        if (start[i] != end[j]) {
          return false;
        }
        char c = start[i];
        if ((c == 'L' && i < j) || (c == 'R' && i > j)) {
          return false;
        }
        i++;
        j++;
      }
    }
    while (i < n) {
      if (start[i] != 'X') {
        return false;
      }
      i++;
    }
    while (j < n) {
      if (end[j] != 'X') {
        return false;
      }
      j++;
    }
    return true;
  }
};

int main() {
  assert(Solution().canTransform("RXXLRXRXL", "XRLXXRRLX"));
  return 0;
}