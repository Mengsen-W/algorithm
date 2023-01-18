/*
 * @Date: 2022-05-13 09:27:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-13 09:31:52
 * @FilePath: /algorithm/01.05_one_edit_away/one_edit_away.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool oneEditAway(string first, string second) {
    int m = first.size(), n = second.size();
    if (n - m == 1) {
      return oneInsert(first, second);
    } else if (m - n == 1) {
      return oneInsert(second, first);
    } else if (m == n) {
      bool foundDifference = false;
      for (int i = 0; i < m; i++) {
        if (first[i] != second[i]) {
          if (!foundDifference) {
            foundDifference = true;
          } else {
            return false;
          }
        }
      }
      return true;
    } else {
      return false;
    }
  }

  bool oneInsert(string shorter, string longer) {
    int length1 = shorter.size(), length2 = longer.size();
    int index1 = 0, index2 = 0;
    while (index1 < length1 && index2 < length2) {
      if (shorter[index1] == longer[index2]) {
        index1++;
      }
      index2++;
      if (index2 - index1 > 1) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  assert(Solution().oneEditAway("pale", "ple"));
  assert(!Solution().oneEditAway("pales", "pal"));
}