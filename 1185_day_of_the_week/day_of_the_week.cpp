/*
 * @Date: 2022-01-03 01:02:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-03 01:43:47
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  string dayOfTheWeek(int day, int month, int year) {
    vector<string> week = {"Monday", "Tuesday",  "Wednesday", "Thursday",
                           "Friday", "Saturday", "Sunday"};
    vector<int> monthDays = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30};
    /* 输入年份之前的年份的天数贡献 */
    int days = 365 * (year - 1971) + (year - 1969) / 4;
    /* 输入年份中，输入月份之前的月份的天数贡献 */
    for (int i = 0; i < month - 1; ++i) {
      days += monthDays[i];
    }
    if ((year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)) && month >= 3) {
      days += 1;
    }
    /* 输入月份中的天数贡献 */
    days += day;
    return week[(days + 3) % 7];
  }
};

int main() {
  assert(Solution().dayOfTheWeek(31, 8, 2019) == "Saturday");
  assert(Solution().dayOfTheWeek(18, 7, 1999) == "Sunday");
  assert(Solution().dayOfTheWeek(15, 8, 1993) == "Sunday");
  assert(Solution().dayOfTheWeek(29, 2, 2016) == "Monday");
}