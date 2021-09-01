/*
 * @Date: 2021-09-01 13:49:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-01 15:18:45
 */

#include <cassert>
#include <iostream>
#include <string>

using namespace std;

class Solution {
 public:
  int compareVersion(string version1, string version2) {
    int n = version1.length(), m = version2.length();
    int i = 0, j = 0;
    while (i < n || j < m) {
      int x = 0;
      for (; i < n && version1[i] != '.'; ++i) {
        x = x * 10 + version1[i] - '0';
      }
      ++i;  // 跳过点号
      int y = 0;
      for (; j < m && version2[j] != '.'; ++j) {
        y = y * 10 + version2[j] - '0';
      }
      ++j;  // 跳过点号
      cout << "x = " << x << " y = " << y << endl;
      if (x != y) {
        return x > y ? 1 : -1;
      }
    }
    return 0;
  }
};

int main() {
  {
    string version1 = "1.01";
    string version2 = "1.001";
    assert(Solution().compareVersion(version1, version2) == 0);
  }
  {
    string version1 = "1.0";
    string version2 = "1.0.0";
    assert(Solution().compareVersion(version1, version2) == 0);
  }
  {
    string version1 = "0.1";
    string version2 = "1.1";
    assert(Solution().compareVersion(version1, version2) == -1);
  }
  {
    string version1 = "1.0.1";
    string version2 = "1";
    assert(Solution().compareVersion(version1, version2) == 1);
  }
  {
    string version1 = "7.5.2.4";
    string version2 = "7.5.3";
    assert(Solution().compareVersion(version1, version2) == -1);
  }
}
