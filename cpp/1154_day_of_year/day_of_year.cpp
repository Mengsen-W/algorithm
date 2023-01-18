/*
 * @Date: 2021-12-21 01:20:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-21 01:45:21
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int dayOfYear(string date) {
    int year = stoi(date.substr(0, 4));
    int month = stoi(date.substr(5, 2));
    int day = stoi(date.substr(8, 2));

    int amount[] = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    if (year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)) {
      ++amount[1];
    }

    int ans = 0;
    for (int i = 0; i < month - 1; ++i) {
      ans += amount[i];
    }
    return ans + day;
  }
};

int main() {
  assert(Solution().dayOfYear("2019-01-09") == 9);
  assert(Solution().dayOfYear("2019-02-10") == 41);
  assert(Solution().dayOfYear("2003-03-01") == 60);
  assert(Solution().dayOfYear("2004-03-01") == 61);
}