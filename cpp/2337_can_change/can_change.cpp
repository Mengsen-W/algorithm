/*
 * @Date: 2023-08-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-21
 * @FilePath: /algorithm/cpp/2337_can_change/can_change.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canChange(string start, string target) {
    int n = start.length();
    int i = 0, j = 0;
    while (i < n && j < n) {
      while (i < n && start[i] == '_') {
        i++;
      }
      while (j < n && target[j] == '_') {
        j++;
      }
      if (i < n && j < n) {
        if (start[i] != target[j]) {
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
      if (start[i] != '_') {
        return false;
      }
      i++;
    }
    while (j < n) {
      if (target[j] != '_') {
        return false;
      }
      j++;
    }
    return true;
  }
};

int main() {
  vector<tuple<string, string, bool>> tests{
      {"_L__R__R_", "L______RR", true},
      {"R_L_", "__LR", false},
      {"_R", "R_", false},
  };

  for (auto &[start, target, expected] : tests) {
    assert(Solution().canChange(start, target) == expected);
  }
}